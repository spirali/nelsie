from typing import TypeVar, Sequence

T = TypeVar("T")

Step = int | tuple[int]

type Sv[T] = T | dict[Step, T]


def at_step(obj: Sv[T], step: Step) -> T:
    if isinstance(obj, dict):
        raise Exception("TODO")
    return obj


def extract_steps(obj: Sv[T], out: set[Step]):
    if isinstance(obj, dict):
        for key in obj:
            out.add(key)


def in_steps(objs: Sequence[T]) -> dict[Step, T]:
    raise NotImplementedError
