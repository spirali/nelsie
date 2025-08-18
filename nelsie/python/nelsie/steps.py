from typing import TypeVar, Generic, Sequence

from .utils import check_is_bool
from . import nelsie as nelsie_rs
from functools import cmp_to_key

LoadedImage = nelsie_rs.LoadedImage

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


def step_compare(a, b):
    if isinstance(a, int) ^ isinstance(b, int):
        if isinstance(a, int):
            a = (a,)
        if isinstance(b, int):
            b = (b,)
    if a < b:
        return -1
    elif a > b:
        return 1
    else:
        return 0


class StepVal(Generic[T]):
    def __init__(self, init_value: T | None = None, named_steps=None, init_values=None):
        if init_values is not None:
            self.values = init_values
        elif init_value is not None:
            self.values = {1: init_value}
        else:
            self.values = {}
        self.named_steps = named_steps

    def at(self, step: Step, value: T) -> "StepVal[T]":
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

    def map_fn(self, fn: callable(T)) -> "StepVal[T]":
        values = {}
        for step, value in self.values.items():
            values[step] = fn(value)
        return StepVal(values, self.named_steps)

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

    def __repr__(self):
        v = f"<StepVal "
        for step, value in sorted(self.values.items(), key=cmp_to_key(step_compare)):
            v += f"{step}={value!r}, "
        v += ">"
        return v


type Sv[T] = T | StepVal[T]
type Sn[T] = Sv[T | None]


def sv_check(obj, check_fn):
    if isinstance(obj, StepVal):
        obj.call(check_fn)
    else:
        check_fn(obj)


def sn_check(obj, check_fn):
    if isinstance(obj, StepVal):
        obj.call_if_not_none(check_fn)
    elif obj is not None:
        check_fn(obj)


def sn_apply(obj, apply_fn):
    if isinstance(obj, StepVal):
        obj.call_if_not_none(apply_fn)
    if obj is not None:
        apply_fn(obj)


def sn_map(obj, map_fn):
    if isinstance(obj, StepVal):
        obj.map(map_fn)
    if obj is not None:
        return map_fn(obj)
    return None


def get_step(obj: Sn[T], step: Step, default_value: T | None = None) -> T:
    if isinstance(obj, StepVal):
        return obj.get_step(step, default_value)
    if obj is None:
        return default_value
    else:
        return obj


type BoolStepDef = bool | StepVal[bool] | str | int


def parse_bool_steps(value: BoolStepDef) -> Sn[bool]:
    if isinstance(value, bool):
        return value
    if isinstance(value, str):
        steps, named_steps = nelsie_rs.parse_bool_steps(value)
        return StepVal(init_values=steps, named_steps=named_steps)
    if isinstance(value, StepVal):
        value.call(check_is_bool)
        return value
    if isinstance(value, int):
        return StepVal(init_values={value: True, value + 1: False})
    raise Exception(f"Invalid bool step definition: {value!r}")
