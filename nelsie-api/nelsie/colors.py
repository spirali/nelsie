def check_color(value: str) -> str:
    if value is None or isinstance(value, str):
        # TODO: Validate color
        return value
    raise ValueError(f"Invalid color definition: {value!r}")
