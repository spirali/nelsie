"""
This plugin allows you to use Belsie code in MkDocs documentation.

Use a code block with language `nelsie` in Markdown.
Each slide has access to `slides` and `slide` variables.
You can use steps in the code block.

You can optionally use the following parameters:
- width=<int>: Width of the resulting image.
- height=<int>: Height of the resulting image.
- type=<lib, render>: If `type` is `lib`, do not render the code block as a slide, but use it as
a library function for futher slides on the page.
- debug=<yes, no>: Whether to use debug_boxes.

Example:
```nelsie,width=300,height=300
slide.text("Hello world")
```
"""

import contextlib
import os
import base64
import uuid
from typing import List

from mkdocs.config.config_options import Choice
from mkdocs.plugins import BasePlugin


def is_fence_delimiter(line):
    line = line.strip()
    return line.startswith("```") or line.startswith("~~~")


def get_fence_char(header):
    return "`" if header[0] == "`" else "~"


def parse_fence_header(header):
    fence_char = get_fence_char(header)
    items = header.strip(f" {fence_char}").split(",")
    lang = items[0]

    def get_kv(item):
        vals = item.split("=")
        if len(vals) == 1:
            return (vals[0], "yes")
        return vals

    args = dict(get_kv(item) for item in items[1:])
    return (lang, args)


def nelsie_to_python_header(header):
    fence_char = get_fence_char(header) * 3
    start = header.index(f"{fence_char}nelsie")
    return header[:start] + f"{fence_char}python"


class CodeContext:
    def __init__(self):
        self.lib = []

    def add_lib_code(self, code_lines):
        self.lib += code_lines

    def get_lib_code(self):
        return "\n".join(self.lib)


def trim_indent(lines):
    min_indent = min(len(line) - len(line.lstrip()) for line in lines)
    return [line[min_indent:] for line in lines]


@contextlib.contextmanager
def change_cwd(directory: str):
    cwd = os.getcwd()
    os.chdir(directory)
    try:
        yield
    finally:
        os.chdir(cwd)


def render_slide(
    code: List[str],
    docs_dir: str,
    ctx: CodeContext,
    width: int,
    height: int,
    debug_boxes: bool,
    render_format: str,
) -> str:
    code = [line for line in code if "#!IGNORE" not in line]
    code = "\n".join(trim_indent(code))
    template = f"""
from nelsie import SlideDeck, TextStyle, Arrow, Stroke, InSteps, Path
deck = SlideDeck()
{code}
""".strip()

    locals = {}
    code_object = compile(template, "nelsie_render.py", "exec")

    with change_cwd(docs_dir):
        exec(code_object, locals)  # Sorry
    deck = locals["deck"]
    result = deck.render(None, "png")
    uid = uuid.uuid4().hex
    n_pages = len(result)
    button_style = "border: 1px solid black; cursor: pointer; background-color: lightgray; padding: 0.5em; margin: 0.5em; border-radius: 10px;"

    update_js = f"document.getElementById('{uid}_current').textContent = c_{uid}; document.getElementById('{uid}_img').src = data_{uid}[c_{uid} - 1]"
    on_click1 = f"c_{uid} -= 1; if (c_{uid} < 1) c_{uid} = 1; {update_js}"
    on_click2 = f"c_{uid} += 1; if (c_{uid} > {n_pages}) c_{uid} = {n_pages}; {update_js}"
    data_array = [f'"data:image/png;base64,{base64.b64encode(data).decode()}"' for _, _, data in result]
    inital_page = 1

    if n_pages > 1:
        return f"""
        <div>
        <script type='text/javascript'>
        var data_{uid} = [{",".join(data_array)}];
        var c_{uid} = {inital_page};
        </script>
        <div>
        <button type="button" style="{button_style}" onClick="{on_click1}">Prev</button>
        <span id="{uid}_current">1</span>/{n_pages}
        <button type="button" style="{button_style}" onClick="{on_click2}">Next</button>
        </div>
        <img id="{uid}_img" style="border: 1px solid black" width="300"/>
        <script>{update_js}</script>
        </div>
        """.strip()
    else:
        return f"""
        <div style="padding-bottom: 1.5em"><img style="border: 1px solid black" width="300" src={data_array[0]}/></div>
        """.strip()


def iterate_fences(src: str, handle_fence):
    lines = []
    fence_content = []
    inside_fence = False
    fence_header = None
    nested = 0

    for line in src.splitlines(keepends=False):
        if is_fence_delimiter(line):
            if inside_fence:
                # Handle nested fenced code blocks
                if len(line.strip()) != 3:
                    nested += 1
                    fence_content.append(line)
                    continue
                elif nested > 0:
                    nested -= 1
                    fence_content.append(line)
                    continue
                assert nested == 0
                assert fence_header is not None
                header, after, fence_content = handle_fence(fence_header, fence_content)
                lines.append(header)
                lines += fence_content
                lines.append(line)
                lines += after or []

                fence_content = []
                inside_fence = False
                fence_header = None
            else:
                inside_fence = True
                fence_header = line
        elif inside_fence:
            fence_content.append(line)
        else:
            lines.append(line)

    return "\n".join(lines)


def strip_ignore_suffix(lines):
    result = []
    for line in lines:
        if "#!IGNORE" not in line:
            result.append(line)
        else:
            result.append(line[: line.index("#!IGNORE")].rstrip())
    return result


class NelsiePlugin(BasePlugin):
    config_scheme = (("render_format", Choice(("svg", "png"))),)

    def on_page_markdown(self, src: str, page, config, *args, **kwargs):
        # TODO: use Markdown parser

        render_format = self.config.get("render_format", "svg")
        docs_dir = config["docs_dir"]
        ctx = CodeContext()

        def handle_fence(header, fence_lines):
            lang, args = parse_fence_header(header)
            if lang == "nelsie":
                lines = []
                type = args.get("type", "render")
                if type == "lib":
                    ctx.add_lib_code(fence_lines)
                elif type == "render":
                    width = args.get("width", "300")
                    height = args.get("height", "300")
                    debug_boxes = args.get("debug", "no")

                    lines = render_slide(
                        fence_lines,
                        docs_dir,
                        ctx,
                        width=width,
                        height=height,
                        debug_boxes=debug_boxes == "yes",
                        render_format=render_format,
                    ).splitlines(keepends=False)
                    fence_lines = strip_ignore_suffix(fence_lines)
                return nelsie_to_python_header(header), lines, fence_lines
            return header, [], fence_lines

        return iterate_fences(src, handle_fence)
