from dataclasses import dataclass
from typing import Union, Literal

from .resources import Resources
from .basictypes import Position, Size
from .image import RawImage
from .steps import Step, get_step
from .textstyle import TextStyle, merge_in_step
from .box import Box, TextContent
from .slidedeck import Slide
from . import nelsie as nelsie_rs

@dataclass
class RawBox:
    x: Position
    y: Position
    z_level: int
    width: Size
    height: Size
    bg_color: str | None
    row: bool
    reverse: bool
    children: list["RawBox"]
    content: Union[None, "TextContent", RawImage]


@dataclass
class ToRawContext:
    text_style: TextStyle
    code_style: TextStyle
    shared_data: dict[int, bytes]
    z_level: int = 0

    def update(self, box: "Box", step: Step):
        z_level = get_step(box._z_level, step)
        if z_level is None:
            z_level = self.z_level
        return ToRawContext(
            text_style=merge_in_step(self.text_style, box._text_style, step),
            code_style=merge_in_step(self.code_style, box._code_style, step),
            shared_data=self.shared_data,
            z_level=z_level
        )


def box_to_raw(box: "Box", step: Step, ctx: ToRawContext) -> RawBox:
    if box._text_style is not None or box._code_style is not None or box._z_level is not None:
        ctx = ctx.update(box, step)
    if box._content:
        content = box._content.to_raw(step, ctx)
    else:
        content = None
    return RawBox(
        x=get_step(box._x, step),
        y=get_step(box._y, step),
        z_level=ctx.z_level,
        width=get_step(box._width, step),
        height=get_step(box._height, step),
        bg_color=get_step(box._bg_color, step),
        children=[box_to_raw(child, step, ctx) for child in box._children],
        content=content,
        row=get_step(box._row, step),
        reverse=get_step(box._reverse, step),
    )


class RawPage:
    def __init__(self, root: RawBox, width: float, height: float, bg_color: str):
        self.width = width
        self.height = height
        self.bg_color = bg_color
        self.root = root


class Document:
    def __init__(self, resources: Resources, pages: list[RawPage]):
        self.pages = pages
        self.resources = resources

    def render(self, path: str | None, format: Literal["pdf", "png", "svg"] = "pdf", compression_level: int = 1,
               n_threads: int | None = None):
        nelsie_rs.render(self.resources._resources, self.pages, path, format, compression_level,
                         n_threads)


def slide_to_raw(slide: Slide, step: Step, deck: "SlideDeck", shared_data: dict[int, bytes]) -> RawPage:
    width = get_step(slide.width, step, deck.width)
    height = get_step(slide.height, step, deck.height)
    text_style = merge_in_step(deck.text_style, slide.text_style, step)
    code_style = merge_in_step(deck.code_style, slide.code_style, step)
    ctx = ToRawContext(text_style, code_style, shared_data)
    root = RawBox(
        x=None,
        y=None,
        z_level=0,
        width=width,
        height=height,
        bg_color=None,
        children=[box_to_raw(child, step, ctx) for child in slide.children],
        content=None,
        row=False,
        reverse=False,
    )
    return RawPage(
        width=width,
        height=height,
        bg_color=get_step(slide.bg_color, step, deck.bg_color),
        root=root,
    )
