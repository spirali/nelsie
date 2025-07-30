from typing import TypeVar, Generic, Sequence

from nelsie.utils import check_is_bool
from . import nelsie as nelsie_rs

T = TypeVar("T")

Step = int | tuple[int]


def check_step(obj):
    if isinstance(obj, int):
        if obj < 1:
            raise Exception("Invalid step; must be >= 1")
        return
    if isinstance(obj, tuple) and len(obj) and all(isinstance(i, int) and i >= 0 for i in obj):
        return
    raise Exception("Invalid step")


def step_lte(a, b):
    if isinstance(a, int) ^ isinstance(b, int):
        if isinstance(a, int):
            a = (a,)
        if isinstance(b, int):
            b = (b,)
    return a <= b


class InSteps(Generic[T]):

    def __init__(self, init_values: dict[Step, T] | Sequence[T] | None = None, named_steps=None):
        if init_values is None:
            self.values = {}
        elif isinstance(init_values, dict):
            for step in init_values:
                check_step(step)
            self.values = init_values
        else:
            self.values = {i: v for i, v in enumerate(init_values, 1)}
        self.named_steps = named_steps

    def s(self, step: Step, value: T) -> "InSteps[T]":
        check_step(step)
        self.values[step] = value
        return self

    def call(self, fn: callable(T)):
        for value in self.values.values():
            fn(value)

    def call_if_not_none(self, fn: callable(T)):
        for value in self.values.values():
            if value is not None:
                fn(value)

    def get_step(self, step: Step, default_value: T | None = None) -> T | None:
        if step in self.values:
            return self.values[step]
        result = default_value
        current_step = 0
        for i in self.values:
            if step_lte(i, step) and step_lte(current_step, i):
                result = self.values[i]
                current_step = i
        return result


def s(value: T = None) -> InSteps[T]:
    result = InSteps()
    if value is not None:
        result.s(1, value)
    return result


type Sv[T] = T | InSteps[T]
type Sn[T] = Sv[T | None]


def sv_check(obj, check_fn):
    if isinstance(obj, InSteps):
        obj.call(check_fn)
    else:
        check_fn(obj)


def sn_check(obj, check_fn):
    if isinstance(obj, InSteps):
        obj.call_if_not_none(check_fn)
    elif obj is not None:
        check_fn(obj)


def sn_apply(obj, apply_fn):
    if obj is not None:
        apply_fn(obj)


def get_step(obj: Sn[T], step: Step, default_value: T | None = None) -> T:
    if isinstance(obj, InSteps):
        return obj.get_step(step, default_value)
    return obj


containers = (list, tuple, set)


def extract_steps(obj, out: set[Step]):
    if obj is None:
        return
    if isinstance(obj, InSteps):
        if obj.named_steps is not None:
            out.update(obj.named_steps.keys())
        else:
            out.update(obj.values.keys())
        return
    if isinstance(obj, containers):
        for o in obj:
            extract_steps(o, out)
    if isinstance(obj, dict):
        for o in obj.values():
            extract_steps(o, out)
    elif hasattr(obj, '__dict__'):
        for o in obj.__dict__.values():
            extract_steps(o, out)


type BoolStepDef = bool | InSteps[bool] | str


def parse_bool_steps(value: BoolStepDef) -> Sn[bool]:
    if isinstance(value, bool):
        return value
    if isinstance(value, str):
        steps, named_steps = nelsie_rs.parse_bool_steps(value)
        return InSteps(steps, named_steps)
    if isinstance(value, InSteps):
        value.call(check_is_bool)
        return value
    raise Exception("Invalid bool step definition")
