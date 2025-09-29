from dataclasses import dataclass

from .textstyle import TextStyle, merge_in_step
from .steps import Sv, Sn, Step, get_step
from .basictypes import TextAlign


@dataclass
class RawText:
    text: str
    style: TextStyle
    align: TextAlign = "start"
    syntax_language: str | None = None
    syntax_theme: str | None = None
    named_styles: dict[str, TextStyle] | None = None
    style_delimiters: str | None = None


@dataclass
class TextContent:
    text: Sv[str]
    style: Sn[TextStyle | str]
    align: Sv[TextAlign]
    is_code: bool
    parse_styles: bool
    style_delimiters: str
    syntax_language: Sn[str]
    syntax_theme: Sn[str]

    def to_raw(self, step: Step, ctx) -> RawText | None:
        text = get_step(self.text, step)
        if text is None:
            return None
        if self.parse_styles:
            style_names = ctx.get_style_names()
            text_styles = {}
            for name in style_names:
                if name in text:
                    style = ctx.get_text_style(name, step)
                    if style is None:
                        style = TextStyle()
                    text_styles[name] = style
        else:
            text_styles = None

        style = get_step(self.style, step)
        if isinstance(style, str):
            name = style
            style = ctx.get_text_style(style, step)
            if style is None:
                raise Exception(f"Style '{name}' not found.")
        if self.is_code:
            code_style = ctx.get_text_style("code", step)
            style = merge_in_step(code_style, style, step)
        default_style = ctx.get_text_style("default", step)
        style = merge_in_step(default_style, style, step)
        raw_text = RawText(
            text=get_step(self.text, step),
            style=style,
            align=get_step(self.align, step),
            syntax_language=get_step(self.syntax_language, step, ctx.code_language)
            if self.is_code
            else None,
            syntax_theme=get_step(self.syntax_theme, step, ctx.code_theme)
            if self.is_code
            else None,
            named_styles=text_styles,
            style_delimiters=self.style_delimiters if self.parse_styles else None,
        )
        return raw_text

    def traverse_tree(self, shared_data, steps):
        pass
