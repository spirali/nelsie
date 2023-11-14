from typing import Callable, TypeVar

from nelsie.export import ExportStepValue, ExportConstStepValue, ExportComplexStepValue
from .insteps import InSteps


T = TypeVar("T")
S = TypeVar("S")


def export_step_value(
    obj: InSteps[T] | T, slide, export_value_fn: Callable[[T], S] | None = None
) -> ExportStepValue[S]:
    if isinstance(obj, InSteps):
        slide.update_min_steps(obj.n_steps)
        if export_value_fn:
            obj = obj.map(export_value_fn)
        values = obj.values
        if len(values) == 1:
            return ExportConstStepValue(values[0])
        else:
            return ExportComplexStepValue(values)
    else:
        if export_value_fn:
            return ExportConstStepValue(export_value_fn(obj))
        else:
            return ExportConstStepValue(obj)
