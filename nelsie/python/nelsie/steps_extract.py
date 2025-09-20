from .image import ImageContent
from .steps import StepVal, Step
from .textstyle import TextStyle
from .text import TextContent
from .box import Box
from .slidedeck import Slide
from .shapes import Path, Rect, Oval, Stroke, Arrow

containers = (list, tuple, set)
known_classes = (
    Box,
    TextContent,
    TextStyle,
    ImageContent,
    Path,
    Rect,
    Oval,
    Stroke,
    Arrow,
)


def extract_steps(obj, out: set[Step]):
    if obj is None:
        return
    if isinstance(obj, StepVal):
        if obj.named_steps is not None:
            out.update(obj.named_steps)
        else:
            out.update(obj.values.keys())
        return
    if isinstance(obj, containers):
        for o in obj:
            extract_steps(o, out)
        return
    if isinstance(obj, dict):
        for o in obj.values():
            extract_steps(o, out)
        return
    if isinstance(obj, Slide):
        d = obj.__dict__
        for k in d:
            if k != "subslides":
                extract_steps(d[k], out)
        return
    if isinstance(obj, known_classes):
        for o in obj.__dict__.values():
            extract_steps(o, out)
