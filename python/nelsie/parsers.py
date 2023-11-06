import re


SIZE_REGEXP = re.compile(r"^(\d+(?:\.\d+)?)(%)?$")


def parse_size(value):
    if value == "auto":
        return value
    if isinstance(value, (int, float)):
        return {"points": value}
    if isinstance(value, str):
        match = SIZE_REGEXP.match(value)
        if match:
            number, percent = match.groups()
            return {"percent": float(number)} if percent else {"points": float(number)}
    raise ValueError(f"Invalid size definition: {value!r}")


def parse_color(value):
    if value is None or isinstance(value, str):
        # TODO: Validate color
        return value
    raise ValueError(f"Invalid color definition: {value!r}")


def check_type(obj, cls):
    if isinstance(obj, cls):
        return obj
    raise TypeError(f"Expected {cls} got {type(obj)}")
