from typing import TypeVar, Generic

T = TypeVar("T")

Step = int | tuple[int]

class InSteps(Generic[T]):

    def __init__(self):
        self.values = {}

    def s(self, value: T, from_step: Step = 1, to_step: Step | None = None) -> "InSteps[T]":
        self.values[from_step] = value
        if to_step is not None and not to_step in self.values:
            self.values[to_step] = value
        return self


def s(value: T, from_step: Step = 1, to_step: Step | None = None) -> InSteps[T]:
    return InSteps().s(value, from_step, to_step)


type Sv[T] = T | InSteps[T]
type Sn[T] = Sv[T | None]

def get_step(obj: Sn[T], step: Step, default_value: T | None = None) -> T:
    if isinstance(obj, InSteps):
        raise Exception("TODO")
    return obj


def extract_steps(obj: Sv[T], out: set[Step]):
    return
