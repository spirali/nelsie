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


TextAlign = Literal["start", "center", "end"]


AlignItems = Literal[
    "start",
    "end",
    "flex-start",
    "flex-end",
    "flex-end",
    "center",
    "stretch",
    "baseline",
]

AlignContent = Literal[
    "start",
    "end",
    "flex-start",
    "flex-end",
    "flex-end",
    "center",
    "stretch",
    "space-between",
    "space-evenly",
    "space-around",
]

FlexWrap = Literal["nowrap", "wrap", "wrap-reverse"]


def parse_debug_layout(value: bool | str) -> str | None:
    if value is True:
        return "#ff00ff"
    elif not value:
        return None
    return typing.cast(str, value)
