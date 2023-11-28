from typing import Sequence

from nelsie.export import StyledText
from nelsie.steps.insteps import InSteps, to_steps, zip_in_steps

from .manager import TextStyleManager, update_stepped_text_style
from .textstyle import TextStyle
from .texttypes import StyledLine, StyledSpan


def _find_first(string, start, c1, c2) -> int:
    p1 = string.find(c1, start)
    p2 = string.find(c2, start)
    if p1 == -1:
        return p2
    if p2 == -1:
        return p1
    return min(p1, p2)

def len_in_bytes(string):
    return len(string.encode("utf-8"))


def parse_styled_text(
    text: str,
    delimiters: Sequence[str],
    base_style: TextStyle,
    style_manager: TextStyleManager,
) -> InSteps[StyledText]:
    start_sequence, start_block, end_block = delimiters
    assert start_sequence != start_block
    assert start_sequence != end_block

    styled_lines: list[StyledLine] = []

    style_name_stack: list[str] = []

    style_names: list[list[str]] = []
    styles: list[TextStyle] = []

    def add_chunk(added_text):
        if added_text:
            try:
                style_idx = style_names.index(style_name_stack)
            except ValueError:
                style_idx = len(style_names)
                style_names.append(style_name_stack[:])
                style = base_style
                for name in style_name_stack:
                    style = update_stepped_text_style(
                        style, style_manager.get_style(name)
                    )
                styles.append(style)
            spans.append(StyledSpan(len_in_bytes(raw_line), len_in_bytes(added_text), style_idx))
        return added_text

    for line_idx, line in enumerate(text.split("\n")):
        spans: list[StyledSpan] = []
        raw_line = ""
        buffer = ""
        last_pos = 0
        while True:
            if style_name_stack:
                pos = _find_first(line, last_pos, start_sequence, end_block)
            else:
                pos = line.find(start_sequence, last_pos)
            if pos == -1:
                raw_line += add_chunk(buffer + line[last_pos:])
                break

            buffer += line[last_pos:pos]

            if line[pos] == start_sequence:
                if pos + 1 < len(line) and line[pos + 1] == start_sequence:
                    buffer += start_sequence
                    last_pos = pos + 2
                    continue
                raw_line += add_chunk(buffer)
                buffer = ""
                block_pos = line.find(start_block, pos)
                if block_pos == -1:
                    raise ValueError(f"Invalid style format on line {line_idx + 1}")
                style_name = line[pos + 1 : block_pos]

                style_name_stack.append(style_name)
                last_pos = block_pos + 1
            else:  # line[pos] == end_block
                raw_line += add_chunk(buffer)
                buffer = ""
                style_name_stack.pop()
                last_pos = pos + 1

        styled_lines.append(StyledLine(spans=spans, text=raw_line))
        # abs_pos_index += len(raw_line) + 1  # Becuase of \n

    def make_styled_text(styles):
        return StyledText(
            styled_lines=styled_lines,
            styles=styles,
            default_font_size=base_style.size,
            default_line_spacing=base_style.line_spacing,
        )

    return zip_in_steps([to_steps(style) for style in styles]).map(make_styled_text)
