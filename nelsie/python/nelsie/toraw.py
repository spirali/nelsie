from dataclasses import dataclass

from .box import Box
from .doc import RawBox
from .steps import Step, get_step
from .textstyle import TextStyle, merge_in_step


@dataclass
class ToRawContext:
    text_style: TextStyle
    code_style: TextStyle
    shared_data: dict[int, bytes]

    def update(self, box: "Box", step: Step):
        return ToRawContext(
            text_style=merge_in_step(self.text_style, box._text_style, step),
            code_style=merge_in_step(self.code_style, box._code_style, step)
        )


def box_to_raw(box: Box, step: Step, ctx: ToRawContext) -> RawBox:
    if box._text_style is not None or box._code_style is not None:
        ctx = ctx.update(box)
    if box._content:
        content = box._content.to_raw(step, ctx)
    else:
        content = None
    return RawBox(
        x=get_step(box._x, step),
        y=get_step(box._y, step),
        width=get_step(box._width, step),
        height=get_step(box._height, step),
        bg_color=get_step(box._bg_color, step),
        children=[box_to_raw(child, step, ctx) for child in box._children],
        content=content,
    )
