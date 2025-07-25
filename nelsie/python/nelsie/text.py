from dataclasses import dataclass

from .textstyle import TextStyle
from .steps import Sv, Sn, Step, get_step
from .basictypes import TextAlign

@dataclass
class TextContent:
    text: Sv[str]
    style: Sn[TextStyle]
    align: Sv[TextAlign]
    syntax_language: Sn[str]
    syntax_theme: Sn[str]

    def get_step(self, step: Step, default_style: TextStyle) -> TextContent:
        return TextContent(
            text=get_step(self.text, step),
            style=get_step(self.style, step, default_style),
            align=get_step(self.align, step),
            syntax_language=get_step(self.syntax_language, step),
            syntax_theme=get_step(self.syntax_theme, step),
        )
