from typing import TypeVar, Generic

T = TypeVar("T")


class StepValue(Generic[T]):
    pass


def to_step_value(obj, parser=None):
    return {"const": parser(obj)}
