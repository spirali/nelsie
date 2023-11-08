from dataclasses import dataclass, fields


def unpack_dataclass(obj: dataclass) -> list:
    return [getattr(obj, name) for name in fields(obj)]
