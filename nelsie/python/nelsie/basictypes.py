from typing import Literal, cast
import re

from .layoutexpr import LayoutExpr
from .utils import int_or_float

type IntOrFloat = int | float
type Length = IntOrFloat | str
type LengthAuto = Length | Literal["auto"]
type Size = Length | LayoutExpr
type Position = Size

length_pattern = re.compile(r"^\s*\d+\s*%?\s*$")

num_or_expr = (int, float, LayoutExpr)


def check_position(obj):
    if obj is None or isinstance(obj, num_or_expr):
        return
    if isinstance(obj, str):
        if length_pattern.match(obj):
            return
    raise Exception("Invalid position definition")


def check_size(obj):
    if obj is None or isinstance(obj, num_or_expr):
        return
    if isinstance(obj, str):
        if length_pattern.match(obj):
            return
    raise Exception("Invalid size definition")


def check_length(obj):
    if isinstance(obj, int_or_float):
        return
    if isinstance(obj, str):
        if length_pattern.match(obj):
            return
    raise Exception("Invalid position definition")


def check_length_auto(obj):
    if isinstance(obj, int_or_float):
        return
    if isinstance(obj, str):
        if obj == "auto":
            return
        if length_pattern.match(obj):
            return
    raise Exception("Invalid position definition")


TextAlign = Literal["start", "center", "end"]

TEXT_ALIGN_VALUES = ("start", "center", "end")


def check_text_align(align):
    if align not in TEXT_ALIGN_VALUES:
        raise ValueError("Invalid text align value")


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
