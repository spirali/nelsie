from dataclasses import dataclass

from .textstyle import TextStyle, merge_in_step
from .steps import Sv, Sn, Step, get_step
from .basictypes import TextAlign


@dataclass
class TextContent:
    text: Sv[str]
    style: Sn[TextStyle]
    align: Sv[TextAlign]
    syntax_language: Sn[str]
    syntax_theme: Sn[str]

    def to_raw(self, step: Step, ctx) -> "TextContent":
        style = merge_in_step(ctx.text_style, self.style, step)
        return TextContent(
            text=get_step(self.text, step),
            style=style,
            align=get_step(self.align, step),
            syntax_language=get_step(self.syntax_language, step),
            syntax_theme=get_step(self.syntax_theme, step),
        )

    def traverse_tree(self, shared_data):
        pass
