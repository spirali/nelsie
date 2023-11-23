from dataclasses import is_dataclass, fields

BASIC_TYPES = (int, float, str, bool)


def serialize(obj):
    if obj is None or isinstance(obj, BASIC_TYPES):
        return obj
    if is_dataclass(obj):
        result = {f.name: serialize(getattr(obj, f.name)) for f in fields(obj)}
        if hasattr(obj, "_tag"):
            result["type"] = getattr(obj, "_tag")
        return result
    if isinstance(obj, list):
        return [serialize(child) for child in obj]
    if isinstance(obj, dict):
        return {key: serialize(value) for key, value in obj.items()}
    raise Exception(f"Cannot serialize {obj!r}")
