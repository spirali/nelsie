import enum
import typing
from dataclasses import dataclass
from typing import Literal

from .layoutexpr import LayoutExpr

Length = int | float | str
LengthAuto = Length | Literal["auto"]
Size = Length | None
Position = Length | None | LayoutExpr


@dataclass
class Stroke:
    color: str
    width: float = 1.0
    dash_array: list[float] | None = None
    dash_offset: float = 0.0


@enum.unique
class TextAlign(enum.IntEnum):
    Start = 0
    Center = 1
    End = 2


@enum.unique
class Align(enum.IntEnum):
    Start = 0
    End = 1
    FlexStart = 2
    FlexEnd = 3
    Center = 4
    Stretch = 5

    Baseline = 10  # Allowed in align_items, align_self, justify_self

    # Allowed in justify_content, align_content
    SpaceBetween = 20
    SpaceEvenly = 21
    SpaceAround = 22


@enum.unique
class FlexWrap(enum.IntEnum):
    NoWrap = 0
    Wrap = 1
    WrapReverse = 2


def parse_debug_layout(value: bool | str) -> str | None:
    if value is True:
        value = "#ff00ff"
    elif not value:
        value = None
    return typing.cast(None | str, value)
