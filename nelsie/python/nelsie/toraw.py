from dataclasses import dataclass, field
from typing import Union, Literal
from copy import copy

from .counters import CounterStorage
from .resources import Resources
from .basictypes import (
    Position,
    Size,
    IntOrFloat,
    Length,
    LengthAuto,
    AlignContent,
    AlignItems,
)
from .image import RawImage
from .shapes import RawRect, Stroke
from .steps import Step, get_step, Sn, step_to_str
from .text import RawText
from .textstyle import TextStyle, merge_in_step
from .box import Box, GridOptions
from .slidedeck import Slide
from . import nelsie as nelsie_rs

DEBUG_STEPS_FRAME_HEIGHT = 20
DEBUG_STEPS_FRAME_FONT_SIZE = 15
DEBUG_LAYOUT_Z_LEVEL = 999999
DEBUG_LAYOUT_FONT_SIZE = 8
DEBUG_LAYOUT_DEFAULT_COLOR = "#ff00ff"


@dataclass
class RawBox:
    node_id: int
    children: list[Union["RawBox", "RawRect", "RawText", "RawImage"]]
    x: Position | None = None
    y: Position | None = None
    width: Size | None = None
    height: Size | None = None
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
    flex_grow: float = 0.0
    flex_shrink: float = 1.0
    align_items: AlignItems = None
    align_self: AlignItems = None
    justify_self: AlignItems = None
    align_content: AlignContent = None
    justify_content: AlignContent = None
    gap_x: Length = 0
    gap_y: Length = 0
    grid: GridOptions = None
    border_radius: IntOrFloat = 0
    url: str | None = None


@dataclass
class ToRawContext:
    text_style_stack: list[dict[str, Sn[TextStyle]]]
    code_theme: str
    code_language: str | None
    shared_data: dict[int, bytes]
    debug_layout: str | bool = False
    debug_layout_boxes: list = field(default_factory=list)
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
        return result.get_step(step)

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

    grid = box._grid
    if grid is not None:
        grid = get_step(grid, step)
        if grid is not None:
            grid = GridOptions(
                template_rows=get_step(grid.template_rows, step, ()),
                template_columns=get_step(grid.template_columns, step, ()),
                row=get_step(grid.row, step, "auto"),
                column=get_step(grid.column, step, "auto"),
            )

    debug_layout = box._debug_layout
    if debug_layout is None:
        debug_layout = ctx.debug_layout
    if debug_layout:
        ctx.debug_layout_boxes.append(box)

    return RawBox(
        node_id=id(box),
        x=get_step(box._x, step),
        y=get_step(box._y, step),
        show=get_step(box._show, step, False),
        z_level=ctx.z_level,
        width=get_step(box._width, step),
        height=get_step(box._height, step),
        bg_color=get_step(box._bg_color, step),
        children=children_to_raw(box._children, step, ctx),
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
        flex_grow=get_step(box._flex_grow, step),
        flex_shrink=get_step(box._flex_shrink, step),
        align_items=get_step(box._align_items, step),
        align_self=get_step(box._align_self, step),
        justify_self=get_step(box._justify_self, step),
        align_content=get_step(box._align_content, step),
        justify_content=get_step(box._justify_content, step),
        gap_x=get_step(box._gap_x, step),
        gap_y=get_step(box._gap_y, step),
        border_radius=get_step(box._border_radius, step),
        grid=grid,
        url=get_step(box._url, step),
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
        format: Literal["pdf", "png", "svg", "layout"] = "pdf",
        compression_level: int = 1,
        n_threads: int | None = None,
        progressbar: bool = True,
    ):
        if path is not None:
            path = str(path)
        return nelsie_rs.render(
            self.resources._resources,
            self.pages,
            path,
            format,
            compression_level,
            n_threads,
            progressbar,
        )


def children_to_raw(children, step: Step, ctx: ToRawContext):
    result = []
    for child in children:
        child = get_step(child, step)
        if child is None:
            continue
        if isinstance(child, Box):
            if get_step(child._active, step):
                result.append(box_to_raw(child, step, ctx))
        else:
            raw = child.to_raw(step, ctx)
            if raw is not None:
                result.append(raw)

    return result


def slide_to_raw(
    resources: Resources,
    slide: Slide,
    step: Step,
    deck,
    shared_data: dict[int, bytes],
    current_counter: CounterStorage,
    total_counter: CounterStorage,
) -> RawPage:
    if slide.postprocess_fn:
        slide = slide.postprocess_fn(slide, current_counter, total_counter)
    width = get_step(slide.width, step)
    height = get_step(slide.height, step)
    stack = [deck._text_styles]
    if slide._text_styles is not None:
        stack.append(slide._text_styles)
    ctx = ToRawContext(
        text_style_stack=stack,
        code_theme=deck.default_code_theme,
        code_language=deck.default_code_language,
        shared_data=shared_data,
        debug_layout=slide.debug_layout,
    )
    root = RawBox(
        node_id=id(slide),
        width=width,
        height=height,
        children=children_to_raw(slide.children, step, ctx),
    )
    page = RawPage(
        width=width,
        height=height,
        bg_color=get_step(slide.bg_color, step, deck.bg_color),
        root=root,
    )
    if ctx.debug_layout_boxes:
        temp_doc = Document(resources, [page])
        layout = temp_doc.render(None, "layout")[0]
        for box in ctx.debug_layout_boxes:
            insert_debug_layout_frame(ctx, box, layout, root)
    if slide.debug_steps:
        height += DEBUG_STEPS_FRAME_HEIGHT
        page.root = RawBox(
            node_id=0,
            width=width,
            height=height,
            children=[
                root,
                debug_steps_frame(step),
            ],
        )
    return page


def debug_steps_frame(step: Step) -> RawBox:
    return RawBox(
        node_id=1,
        width="100%",
        height=DEBUG_STEPS_FRAME_HEIGHT,
        bg_color="black",
        children=[
            RawBox(
                node_id=2,
                children=[],
                content=RawText(
                    step_to_str(step),
                    TextStyle(
                        size=DEBUG_STEPS_FRAME_FONT_SIZE,
                        font="monospace",
                        color="white",
                    ),
                ),
            )
        ],
    )


def insert_debug_layout_frame(
    ctx: ToRawContext, box: Box, layout: dict, target_box: RawBox
):
    box_layout = layout[id(box)]
    x = box_layout["x"]
    y = box_layout["y"]
    width = box_layout["width"]
    height = box_layout["height"]

    color = DEBUG_LAYOUT_DEFAULT_COLOR
    if isinstance(ctx.debug_layout, str):
        color = ctx.debug_layout
    if isinstance(box._debug_layout, str):
        color = box._debug_layout

    w = max(1, width)
    h = max(1, height)

    item = RawRect(
        shape=0,
        x1=x,
        y1=y,
        x2=x + w,
        y2=y + h,
        z_level=DEBUG_LAYOUT_Z_LEVEL,
        stroke=Stroke(color=color, width=1, dash_array=[5, 2]),
        fill_color=None,
    )
    target_box.children.append(item)

    text_w = f"{width:.2f}".rstrip("0").rstrip(".")
    text_h = f"{height:.2f}".rstrip("0").rstrip(".")

    if box.name:
        text = f"{box.name} [{text_w}x{text_h}]"
    else:
        text = f"[{text_w}x{text_h}]"

    item = RawBox(
        node_id=id(item),
        children=[],
        z_level=DEBUG_LAYOUT_Z_LEVEL,
        x=x + 1,
        y=y + 1,
        content=RawText(
            text,
            TextStyle(
                size=DEBUG_LAYOUT_FONT_SIZE,
                font="monospace",
                color=color,
            ),
        ),
    )
    target_box.children.append(item)
