from dataclasses import dataclass, fields


def unpack_dataclass(obj: dataclass) -> list:
    return [getattr(obj, field.name) for field in fields(obj)]
