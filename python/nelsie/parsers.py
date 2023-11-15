import re

from .basictypes import Size
from .export import ExportSize, PointsSize, FractionSize, AUTO_SIZE

SIZE_REGEXP = re.compile(r"^(\d+(?:\.\d+)?)(%)?$")


def parse_size(value: Size) -> ExportSize:
    if value == "auto":
        return AUTO_SIZE
    if isinstance(value, (int, float)):
        return PointsSize(float(value))
    if isinstance(value, str):
        match = SIZE_REGEXP.match(value)
        if match:
            number, percent = match.groups()
            return (
                FractionSize(float(number) / 100.0)
                if percent
                else PointsSize(float(number))
            )
    raise ValueError(f"Invalid size definition: {value!r}")


def check_type(obj, cls):
    if isinstance(obj, cls):
        return obj
    raise TypeError(f"Expected {cls} got {type(obj)}")


def check_type_bool(obj):
    return check_type(obj, bool)
