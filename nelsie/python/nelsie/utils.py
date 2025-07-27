from dataclasses import dataclass, fields


def unpack_dataclass(obj: dataclass) -> list:
    return [getattr(obj, field.name) for field in fields(obj)]


def check_is_type(obj, tp):
    if not isinstance(obj, tp):
        raise Exception(f"Expect {tp} but got: {obj!r}")


def check_is_str(obj):
    check_is_type(obj, str)


def check_is_int(obj):
    check_is_type(obj, int)


def check_is_bool(obj):
    check_is_type(obj, bool)
