import re
from typing import Generic, Sequence, TypeVar

StepDef = int | Sequence[int] | str


def to_steps(obj):
    if isinstance(obj, InSteps):
        return obj
    else:
        return InSteps([obj])


T = TypeVar("T")
S = TypeVar("S")


def _extend_values(values: list[T], n: int) -> list[T]:
    if len(values) >= n:
        return values
    result = values[:]
    while len(result) < n:
        result.append(values[-1])
    return result


class InSteps(Generic[T]):
    def __init__(
        self,
        values: Sequence[T] | dict[int, T],
        n_steps: int | None = None,
    ):
        if isinstance(values, Sequence):
            tmp = {}
            prev = None
            for i, v in enumerate(values):
                if i != 0 and v == prev:
                    continue
                tmp[i + 1] = v
                prev = v
            values = tmp
        elif not isinstance(values, dict):
            raise ValueError("Invalid type for values")
        self.values = values
        self.n_steps = n_steps or (max(values.keys()) if values else 1)

    def get(self, step: int) -> T | None:
        v = self.values.get(step)
        if v is not None:
            return v
        if step <= 0:
            return None
        return self.get(step - 1)

    def map(self, fn):
        return InSteps(
            {step: fn(v) for step, v in self.values.items()}, n_steps=self.n_steps
        )

    def key_steps(self):
        return self.values.keys()

    def zip(self, other: "InSteps[S]") -> "InSteps[(S, T)]":
        keys = set(self.key_steps())
        keys.update(other.key_steps())
        return InSteps(
            {step: (self.get(step), other.get(step)) for step in keys},
            n_steps=max(self.n_steps, other.n_steps),
        )


def zip_in_steps(in_steps: Sequence[InSteps[T]]) -> InSteps[T]:
    assert in_steps
    keys = set()
    for s in in_steps:
        keys.update(s.key_steps())
    values = {step: [s.get(step) for s in in_steps] for step in keys}
    n_steps = max(s.n_steps for s in in_steps)
    return InSteps(values, n_steps=n_steps)


STEP_DEF_CHECK_REGEXP = re.compile(
    r"^\s*\d+(?:\s*-\s*\d+)?(?:\s*,\s*\d+(?:\s*-\s*\d+)?)*\+?\s*$"
)
STEP_DEF_SPLIT_REGEXP = re.compile(r"\d+-\d+|\d+")


def parse_steps(obj: StepDef) -> InSteps[bool]:
    if isinstance(obj, bool):
        return InSteps({1: obj})

    plus = False
    if isinstance(obj, int):
        if obj < 1:
            raise ValueError("Step cannot be a zero or negative integer")
        values = [obj]
    elif isinstance(obj, str):
        if not STEP_DEF_CHECK_REGEXP.match(obj):
            raise ValueError("Invalid step format")
        ranges = STEP_DEF_SPLIT_REGEXP.findall(obj)
        values = []
        for item in ranges:
            if "-" in item:
                start, end = map(int, item.split("-"))
                values.extend(range(start, end + 1))
            else:
                values.append(int(item))
        plus = "+" in obj
    elif isinstance(obj, Sequence):
        for s in obj:
            if not isinstance(s, int) or s < 1:
                raise ValueError("Step cannot be a zero or negative integer")
        values = obj
    else:
        raise ValueError("Invalid type for step definition")
    result = {1: False}
    for step in sorted(values):
        if step - 1 not in values:
            result[step] = True
        if step + 1 not in values:
            result[step + 1] = False
    if plus and values:
        del result[step + 1]
    return InSteps(result, max(values))
