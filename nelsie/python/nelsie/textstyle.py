from dataclasses import dataclass
from enum import IntEnum

from .steps import Sn, Step, get_step, sn_check
from .utils import unpack_dataclass, check_is_type, check_is_int_or_float
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


def check_is_non_negative_int_or_float(obj):
    check_is_int_or_float(obj)
    if obj < 0:
        raise Exception("Value has to be non-negative")


def check_is_weight(obj):
    check_is_int_or_float(obj)
    if obj < 1 or obj > 1000:
        raise Exception("Weight has to be in range 1..1000")


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
        sn_check(self.color, check_color)
        sn_check(self.size, check_is_non_negative_int_or_float)
        sn_check(self.line_spacing, check_is_non_negative_int_or_float)
        sn_check(self.weight, check_is_weight)

    def merge(self, other: "TextStyle") -> "TextStyle":
        check_is_text_style(other)
        return TextStyle(
            *[
                b if b is not None else a
                for (a, b) in zip(unpack_dataclass(self), unpack_dataclass(other))
            ]
        )

    def get_step(self, step: Step) -> "TextStyle":
        return TextStyle(
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


def merge_in_step(
    text_style: Sn[TextStyle], other: Sn[TextStyle], step: Step
) -> TextStyle:
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
