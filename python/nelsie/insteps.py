from typing import Generic, Sequence, TypeVar

T = TypeVar("T")
S = TypeVar("S")


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
        self.in_step_values = values
        self.n_steps = n_steps or (max(values.keys()) if values else 1)

    def get(self, step: int) -> T | None:
        v = self.in_step_values.get(step)
        if v is not None:
            return v
        if step <= 0:
            return None
        return self.get(step - 1)

    def map(self, fn):
        return InSteps(
            {step: fn(v) for step, v in self.in_step_values.items()},
            n_steps=self.n_steps,
        )

    def key_steps(self):
        return self.in_step_values.keys()

    def zip(self, other: "InSteps[S]") -> "InSteps[(S, T)]":
        keys = set(self.key_steps())
        keys.update(other.key_steps())
        return InSteps(
            {step: (self.get(step), other.get(step)) for step in keys},
            n_steps=max(self.n_steps, other.n_steps),
        )
