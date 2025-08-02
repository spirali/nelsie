from dataclasses import dataclass

from .textstyle import TextStyle, merge_in_step
from .steps import Sv, Sn, Step, get_step
from .basictypes import TextAlign


@dataclass
class TextContent:
    text: Sv[str]
    style: Sn[TextStyle]
    align: Sv[TextAlign]
    is_code: bool
    syntax_language: Sn[str]
    syntax_theme: Sn[str]

    def to_raw(self, step: Step, ctx) -> "TextContent":
        if self.is_code:
            style = merge_in_step(ctx.code_style, self.style, step)
        else:
            style = self.style
        style = merge_in_step(ctx.text_style, style, step)
        print(ctx.code_theme, self.syntax_theme)
        print(get_step(self.syntax_theme, step, ctx.code_theme))
        return TextContent(
            text=get_step(self.text, step),
            style=style,
            is_code=self.is_code,
            align=get_step(self.align, step),
            syntax_language=get_step(self.syntax_language, step, ctx.code_language),
            syntax_theme=get_step(self.syntax_theme, step, ctx.code_theme),
        )

    def traverse_tree(self, shared_data, steps):
        pass
