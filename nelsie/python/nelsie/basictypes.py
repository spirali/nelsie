from typing import Literal, cast
import re

from .layoutexpr import LayoutExpr

Length = int | float | str
LengthAuto = Length | Literal["auto"]
Size = Length | LayoutExpr
Position = Size

num_or_expr = int | float | LayoutExpr

length_pattern = re.compile(r"^\s*\d+\s*%?\s*$")


def check_position(obj):
    if obj is None or isinstance(obj, num_or_expr):
        return
    if isinstance(obj, str):
        if length_pattern.match(obj):
            return
    raise Exception("Invalid position definition")


def check_size(obj):
    check_position(obj)


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
    return cast(str, value)
