from dataclasses import dataclass
from typing import Union, Literal
from copy import copy

from .resources import Resources
from .basictypes import Position, Size, IntOrFloat, Length, LengthAuto
from .image import RawImage
from .steps import Step, get_step, Sv, Sn
from .text import RawText
from .textstyle import TextStyle, merge_in_step
from .box import Box, TextContent
from .slidedeck import Slide
from . import nelsie as nelsie_rs


@dataclass
class RawBox:
    node_id: int
    x: Position
    y: Position
    width: Size
    height: Size
    children: list["RawBox"]
    show: bool = True
    content: Union[None, RawText, RawImage] = None
    z_level: int = 0
    bg_color: str | None = None
    row: bool = False
    reverse: bool = False
    p_left: Length = 0
    p_right: Length = 0
    p_top: Length = 0
    p_bottom: Length = 0
    m_left: LengthAuto = 0
    m_right: LengthAuto = 0
    m_top: LengthAuto = 0
    m_bottom: LengthAuto = 0


@dataclass
class ToRawContext:
    text_style_stack: list[dict[str, Sn[TextStyle]]]
    code_theme: str
    code_language: str | None
    shared_data: dict[int, bytes]
    z_level: int = 0

    def get_text_style(self, name: str, step: Step):
        result = None
        for text_styles in self.text_style_stack:
            s = text_styles.get(name)
            if s is not None:
                if result is None:
                    result = get_step(s, step)
                else:
                    result = merge_in_step(result, s, step)
        return result

    def get_style_names(self) -> set[str]:
        result = set()
        for text_styles in self.text_style_stack:
            result.update(text_styles.keys())
        return result

    # def update(self, box: "Box", step: Step):
    #     z_level = get_step(box._z_level, step)
    #     if z_level is None:
    #         z_level = self.z_level
    #
    #     text_style = self.text_style
    #     if box._text_style is not None:
    #         text_style = merge_in_step(text_style, box._text_style, step)
    #
    #     code_style = self.code_style
    #     if box._code_style is not None:
    #         text_style = merge_in_step(code_style, box._code_style, step)
    #
    #     return ToRawContext(
    #         text_style=text_style,
    #         code_style=code_style,
    #         code_theme=self.code_theme,
    #         code_language=self.code_language,
    #         shared_data=self.shared_data,
    #         z_level=z_level,
    #     )


def box_to_raw(box: "Box", step: Step, ctx: ToRawContext) -> RawBox:
    if box._text_styles is not None:
        ctx = copy(ctx)
        ctx.text_style_stack.append(box._text_styles)
    z_level = get_step(box._z_level, step)
    if z_level is not None:
        ctx = copy(ctx)
        ctx.z_level = z_level
    if box._content:
        content = box._content.to_raw(step, ctx)
    else:
        content = None

    return RawBox(
        node_id=id(box),
        x=get_step(box._x, step),
        y=get_step(box._y, step),
        show=get_step(box._show, step, False),
        z_level=ctx.z_level,
        width=get_step(box._width, step),
        height=get_step(box._height, step),
        bg_color=get_step(box._bg_color, step),
        children=[box_to_raw(child, step, ctx) for child in box._children if get_step(child._active, step)],
        content=content,
        row=get_step(box._row, step),
        reverse=get_step(box._reverse, step),
        p_left=get_step(box._p_left, step),
        p_right=get_step(box._p_right, step),
        p_top=get_step(box._p_top, step),
        p_bottom=get_step(box._p_bottom, step),
        m_left=get_step(box._m_left, step),
        m_right=get_step(box._m_right, step),
        m_top=get_step(box._m_top, step),
        m_bottom=get_step(box._m_bottom, step),
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

    def render(
        self,
        path: str | None,
        format: Literal["pdf", "png", "svg"] = "pdf",
        compression_level: int = 1,
        n_threads: int | None = None,
    ):
        nelsie_rs.render(self.resources._resources, self.pages, path, format, compression_level, n_threads)


def slide_to_raw(slide: Slide, step: Step, deck: "SlideDeck", shared_data: dict[int, bytes]) -> RawPage:
    width = get_step(slide.width, step)
    height = get_step(slide.height, step)
    stack = [deck._text_styles]
    if slide._text_styles is not None:
        stack.append(slide._text_styles)
    ctx = ToRawContext(stack, deck.default_code_theme, deck.default_code_language, shared_data)
    root = RawBox(
        node_id=id(slide),
        x=None,
        y=None,
        width=width,
        height=height,
        children=[box_to_raw(child, step, ctx) for child in slide.children if get_step(child._active, step)],
    )
    return RawPage(
        width=width,
        height=height,
        bg_color=get_step(slide.bg_color, step, deck.bg_color),
        root=root,
    )
