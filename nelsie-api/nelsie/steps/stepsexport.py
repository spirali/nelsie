from typing import Callable, TypeVar

from nelsie.export import ExportComplexStepValue, ExportConstStepValue, ExportStepValue

from .insteps import InSteps

T = TypeVar("T")
S = TypeVar("S")


def export_step_value(
    obj: InSteps[T] | T,
    slide,
    export_value_fn: Callable[[T], S] | None = None,
    default: T | None = None,
) -> ExportStepValue[S]:
    if isinstance(obj, InSteps):
        slide.update_min_steps(obj.n_steps)
        if export_value_fn:
            obj = obj.map(export_value_fn)
        values = obj.values
        if 1 not in values:
            values[1] = default
        if len(values) == 1 and 1 in values:
            return ExportConstStepValue(values[1])
        else:
            return ExportComplexStepValue(values)
    else:
        if export_value_fn:
            return ExportConstStepValue(export_value_fn(obj))
        else:
            return ExportConstStepValue(obj)
