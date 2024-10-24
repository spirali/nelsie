from dataclasses import dataclass, InitVar
from enum import IntEnum
from typing import Literal

from nelsie.basictypes import Stroke
from nelsie.utils import unpack_dataclass


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
    font_family: str | list[str] | None = None
    color: str | None = None
    size: float | None = None
    line_spacing: float | None = None
    italic: bool | None = None
    stretch: FontStretch | None = None
    underline: bool | None = None
    line_through: bool | None = None

    # 1-1000; 400 = Normal, 700 = Bold
    weight: int | None = None

    # Init only fields
    # These are used as helpers for initializing
    # commonly used attributes
    bold: InitVar[bool | None] = None

    def __post_init__(self, bold: bool | None):
        if self.size is not None:
            assert self.size >= 0
        if self.line_spacing is not None:
            assert self.line_spacing >= 0
        if self.weight is not None:
            assert 1 <= self.weight <= 1000
            if bold is not None:
                raise Exception("Cannot set both `weight` and `bold` when creating a TextStyle")
        if bold is not None:
            # Workaround to set frozen attribute
            super().__setattr__("weight", 700)

    def merge(self, other: "TextStyle") -> "TextStyle":
        assert isinstance(other, TextStyle)
        return TextStyle(
            *[b if b is not None else a for (a, b) in zip(unpack_dataclass(self), unpack_dataclass(other))]
        )


def _data_to_text_style(data):
    stretch = data.get("stretch")
    if stretch is not None:
        data["stretch"] = FontStretch(stretch)
    return TextStyle(**data)
