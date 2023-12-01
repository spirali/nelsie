import typing
from dataclasses import dataclass, field
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
    dash_array: list[float] = field(default_factory=list)
    dash_offset: float = 0.0


def parse_debug_layout(value: bool | str) -> str | None:
    if value is True:
        value = "#ff00ff"
    elif not value:
        value = None
    return typing.cast(None | str, value)
