from typing import Generic, Sequence, TypeVar

T = TypeVar("T")
S = TypeVar("S")


class InSteps(Generic[T]):
    """
    InSteps is a wrapper that allows to set a different values for each step.
    InSteps defines values in "key" steps, in other steps the value remains until it is changed
    another key step.

    Example:
    ```python
    slide.box(..., bg_color=InSteps({1: "green", 4: "red"})
    ```

    Defines "green" background for steps 1, 2, 3; and "red" for step 4 and further.

    InSteps can be also initialized by a list, then it defines values for first `n` steps,
    where `n` is a length of the list. It means that `InSteps(["a", "b", "c"])` is equal to
    `InSteps({1: "a", 2: "b", 3: "c"})`
    """

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

    def get(self, step: int, default: S = None) -> T | None:
        v = self.in_step_values.get(step)
        if v is not None:
            return v
        if step <= 0:
            return default
        return self.get(step - 1, default)

    def map(self, fn):
        return InSteps(
            {step: fn(v) for step, v in self.in_step_values.items()},
            n_steps=self.n_steps,
        )

    def key_steps(self):
        return self.in_step_values.keys()

    def zip(self, other: "InSteps[S]") -> "InSteps[tuple[S, T]]":
        keys = set(self.key_steps())
        keys.update(other.key_steps())
        return InSteps(
            {step: (self.get(step), other.get(step)) for step in keys},
            n_steps=max(self.n_steps, other.n_steps),
        )


def zip_in_steps(values: list[S | InSteps[S]], default: S) -> InSteps[list[S]]:
    keys = set().union(*[x.key_steps() if isinstance(x, InSteps) else (1,) for x in values])
    return InSteps(
        {step: [x.get(step, default) if isinstance(x, InSteps) else x for x in values] for step in keys},
        n_steps=max(x.n_steps if isinstance(x, InSteps) else 1 for x in values),
    )
