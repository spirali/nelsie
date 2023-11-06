from typing import TypeVar, Generic, Sequence
import re


StepDef = int | Sequence[int] | str

#
# class StepCounter:
#     def __init__(self):
#         self.step = 1
#
#     def update(self, value):
#         self.step = max(self.step, value)
#
#     def last(self):
#         return self.step
#
#     def next(self):
#         self.step += 1
#         return self.step


T = TypeVar("T")


class InSteps(Generic[T]):
    def __init__(
        self, values=Sequence[T] | dict[StepDef, T], n_steps: int | None = None
    ):
        if isinstance(values, Sequence):
            if len(values) == 0:
                raise ValueError("Parameter 'values' cannot be an empty list")
            self.values = values
            self.n_steps = n_steps or len(values)
        elif isinstance(values, dict):
            open_keys = 0
            for key in values:
                if isinstance(key, str) and key.rstrip().endswith("+"):
                    open_keys += 1
            if open_keys == 0 or open_keys > 1:
                raise ValueError(
                    "Exactly one step definition has to be unbounded (ends with '+' sign)"
                )
            tmp = []
            max_n_steps = 1
            for key, value in values.items():
                step, n_steps = parse_steps(key)
                max_n_steps = max(max_n_steps, n_steps)
                tmp.append((step, value))
            size = max(len(x[0]) for x in tmp)
            used = [False] * size
            result = [None] * size

            for key, value in tmp:
                if key[-1]:  # This is open step def
                    if len(key) < size:
                        raise ValueError(
                            f"Multiple definitions assigned for step {len(key) + 1}"
                        )
                for i, enabled in enumerate(key):
                    if not enabled:
                        continue
                    if used[i]:
                        raise ValueError(
                            f"Multiple definitions assigned for step {i+1}"
                        )
                    used[i] = True
                    result[i] = value
            unset_steps = [i + 1 for i, u in enumerate(used) if not u]
            if unset_steps:
                raise ValueError(f"Step(s) {unset_steps} have no defined values")
            self.n_steps = n_steps or max_n_steps
            self.values = result
        else:
            raise ValueError("Invalid type for values")

    def expand_values(self, parser):
        if parser is None:
            return self.values
        else:
            return [parser(v) for v in self.values]


def process_step_value(obj, parser=None):
    if isinstance(obj, InSteps):
        return {"steps": obj.expand_values(parser)}, obj.n_steps
    return {"const": parser(obj) if parser is not None else obj}, 1


def _expand_list(seq: Sequence, open: bool) -> (list[bool], int):
    if not seq:
        return [False]
    for value in seq:
        if not isinstance(value, int):
            raise ValueError("Step definition by sequence has to contains integers")
        if value < 1:
            raise ValueError("Step cannot be a zero or negative integer")
    max_value = max(seq)
    result = [False] * (max_value + (1 if not open else 0))
    for value in seq:
        result[value - 1] = True
    return result, max_value


def _expand_single(position: int, open: bool) -> (list[bool], int):
    result = [False] * (position + (1 if not open else 0))
    result[position - 1] = True
    return result, position


STEP_DEF_CHECK_REGEXP = re.compile(
    r"^\s*\d+(?:\s*-\s*\d+)?(?:\s*,\s*\d+(?:\s*-\s*\d+)?)*\+?\s*$"
)
STEP_DEF_SPLIT_REGEXP = re.compile(r"\d+-\d+|\d+")


def parse_steps(obj: StepDef) -> (list[bool], int):
    if isinstance(obj, int):
        if obj < 1:
            raise ValueError("Step cannot be a zero or negative integer")
        return _expand_single(obj, False)

    if isinstance(obj, str):
        if not STEP_DEF_CHECK_REGEXP.match(obj):
            raise ValueError("Invalid step format")
        ranges = STEP_DEF_SPLIT_REGEXP.findall(obj)
        result = []
        for item in ranges:
            if "-" in item:
                start, end = map(int, item.split("-"))
                result.extend(range(start, end + 1))
            else:
                result.append(int(item))
        return _expand_list(result, "+" in obj)
    if isinstance(obj, Sequence):
        return _expand_list(obj, False)
    raise ValueError("Step cannot be a non-positive integer")
