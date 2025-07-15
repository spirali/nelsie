from dataclasses import dataclass, InitVar
from enum import IntEnum

from .steps import Sn, Step, get_step
from .utils import unpack_dataclass, check_is_type
from .nelsie import check_color


class FontStretch(IntEnum):
    UltraCondensed = 1
    ExtraCondensed = 2
    Condensed = 3
    SemiCondensed = 4
    Normal = 5
    SemiExpanded = 6
    Expanded = 7
    ExtraExpanded = 8
    UltraExpanded = 9


@dataclass(frozen=True)
class TextStyle:
    font: Sn[str] = None
    color: Sn[str] = None
    size: Sn[float] = None
    line_spacing: Sn[float] = None
    italic: Sn[bool] = None
    stretch: Sn[FontStretch] = None
    underline: Sn[bool] = None
    line_through: Sn[bool] = None

    # 1-1000; 400 = Normal, 700 = Bold
    weight: Sn[int] = None

    # If True, ignores weight value and forces weight 700
    bold: Sn[bool] = None

    def __post_init__(self):
        if self.color is not None:
            check_color(self.color)
        if self.size is not None:
            assert self.size >= 0
        if self.line_spacing is not None:
            assert self.line_spacing >= 0
        if self.weight is not None:
            assert 1 <= self.weight <= 1000

    def merge(self, other: "TextStyle") -> "TextStyle":
        check_is_text_style(other)
        return TextStyle(
            *[b if b is not None else a for (a, b) in zip(unpack_dataclass(self), unpack_dataclass(other))]
        )

    def get_step(self, step: Step) -> "TextStyle":
        TextStyle(
            font=get_step(self.font, step),
            color=get_step(self.color, step),
            size=get_step(self.size, step),
            line_spacing=get_step(self.line_spacing, step),
            italic=get_step(self.italic, step),
            stretch=get_step(self.stretch, step),
            underline=get_step(self.underline, step),
            line_through=get_step(self.line_through, step),
            weight=get_step(self.weight, step),
            bold=get_step(self.bold, step),
        )


def merge_in_step(text_style: Sn[TextStyle], other: Sn[TextStyle], step: Step) -> TextStyle:
    text_style = get_step(text_style, step)
    other = get_step(other, step)
    if other is None:
        return text_style
    return text_style.merge(other)


def check_is_text_style(obj):
    check_is_type(obj, TextStyle)

str_or_text_style = (TextStyle, str)

def check_is_str_or_text_style(obj):
    check_is_type(obj, str_or_text_style)


DEFAULT_TEXT_STYLE = TextStyle(
    font="sans-serif",
    color="black",
    size=32,
    line_spacing=1.2,
    italic=False,
    stretch=FontStretch.Normal,
    underline=False,
    line_through=False,
    weight=400,
    bold=False,
)

DEFAULT_CODE_STYLE = TextStyle(font="monospace")
