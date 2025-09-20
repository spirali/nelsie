from dataclasses import dataclass
from typing import Union

from .image import (
    PathOrImageData,
    ImageContent,
    check_image_path_or_data,
    normalize_and_watch_image_path,
)
from .steps import (
    Sn,
    Sv,
    sv_check,
    sn_check,
    parse_bool_steps,
    sn_map,
    BoolStepDef,
    StepVal,
    sn_apply,
)

from .basictypes import (
    Position,
    Size,
    check_position,
    check_size,
    TextAlign,
    check_text_align,
    IntOrFloat,
    Length,
    LengthAuto,
    check_length,
    check_length_auto,
    AlignItems,
    AlignContent,
    GridTemplate,
    GridPosition,
    check_align_content,
    check_align_items,
)
from .nelsie import check_color
from .text import TextContent
from .textstyle import TextStyle, check_is_text_style, check_is_str_or_text_style
from .utils import check_is_str, check_is_bool, check_is_int, check_is_int_or_float
from .layoutexpr import LayoutExpr
from .shapes import Rect, Oval, Path, Point


@dataclass
class GridOptions:
    template_rows: Sv[GridTemplate] = ()
    template_columns: Sv[GridTemplate] = ()
    row: Sn[GridPosition] = None
    column: Sn[GridPosition] = None


class BoxBuilderMixin:
    def add(self, box: Union[Path, Rect, Oval, "Box"]):
        """
        Adds Box or a geometry item into the box
        """
        raise NotImplementedError

    def _set_style(self, name: str, style: Sn[TextStyle]):
        raise NotImplementedError

    def _get_style(self, name: str) -> Sn[TextStyle] | None:
        NotImplementedError

    def box(
        self,
        *,
        x: Sn[Position] = None,
        y: Sn[Position] = None,
        z_level: Sn[int] = None,
        show: BoolStepDef = True,
        active: BoolStepDef = True,
        width: Sn[Size] = None,
        height: Sn[Size] = None,
        bg_color: Sn[str] = None,
        row: Sv[bool] = False,
        reverse: Sv[bool] = False,
        p_left: Sv[Length] = 0,
        p_right: Sv[Length] = 0,
        p_top: Sv[Length] = 0,
        p_bottom: Sv[Length] = 0,
        m_left: Sv[LengthAuto] = 0,
        m_right: Sv[LengthAuto] = 0,
        m_top: Sv[LengthAuto] = 0,
        m_bottom: Sv[LengthAuto] = 0,
        flex_grow: Sv[float] = 0.0,
        flex_shrink: Sv[float] = 1.0,
        align_items: Sn[AlignItems] = None,
        align_self: Sn[AlignItems] = None,
        justify_self: Sn[AlignItems] = None,
        align_content: Sn[AlignContent] = None,
        justify_content: Sn[AlignContent] = None,
        gap_x: Sv[Length] = 0,
        gap_y: Sv[Length] = 0,
        grid: Sn[GridOptions] = None,
        border_radius: Sv[IntOrFloat] = 0,
        url: Sn[str] = None,
        name: str = "",
        debug_layout: bool | str | None = None,
    ):
        """
        Create a new child box. See [Box reference](https://spirali.github.io/nelsie/guide/box/) for documentation
        """
        box = Box(
            x=x,
            y=y,
            z_level=z_level,
            show=show,
            active=active,
            width=width,
            height=height,
            bg_color=bg_color,
            row=row,
            reverse=reverse,
            p_left=p_left,
            p_right=p_right,
            p_top=p_top,
            p_bottom=p_bottom,
            m_left=m_left,
            m_right=m_right,
            m_top=m_top,
            m_bottom=m_bottom,
            flex_grow=flex_grow,
            flex_shrink=flex_shrink,
            align_items=align_items,
            align_self=align_self,
            justify_self=justify_self,
            align_content=align_content,
            justify_content=justify_content,
            gap_x=gap_x,
            gap_y=gap_y,
            grid=grid,
            name=name,
            border_radius=border_radius,
            debug_layout=debug_layout,
            url=url,
        )
        self.add(box)
        return box

    def overlay(self, **box_args):
        """
        Create a new box that spans over the box
        """
        box_args.setdefault("x", 0)
        box_args.setdefault("y", 0)
        box_args.setdefault("width", "100%")
        box_args.setdefault("height", "100%")
        return self.box(**box_args)

    def text(
        self,
        text: Sv[str],
        style: Sn[TextStyle] = None,
        *,
        align: Sv[TextAlign] = "start",
        strip: bool = True,
        parse_styles: bool = True,
        style_delimiters: str = "~{}",
        **box_args,
    ):
        if strip and isinstance(text, str):
            text = text.strip()
        sv_check(text, check_is_str)
        sv_check(align, check_text_align)
        sn_check(style, check_is_str_or_text_style)
        box = self.box(**box_args)
        box._content = TextContent(
            text=text,
            style=style,
            align=align,
            is_code=False,
            parse_styles=parse_styles,
            style_delimiters=style_delimiters,
            syntax_language=None,
            syntax_theme=None,
        )
        return box

    def code(
        self,
        text: Sv[str],
        language: Sn[str] = None,
        style: Sn[TextStyle] = None,
        *,
        align: Sv[TextAlign] = "start",
        strip: bool = True,
        theme: Sn[str] = None,
        parse_styles: bool = False,
        style_delimiters: str = "~{}",
        **box_args,
    ):
        if strip and isinstance(text, str):
            text = text.strip()
        sv_check(text, check_is_str)
        sv_check(align, check_text_align)
        sn_check(style, check_is_str_or_text_style)
        sn_check(language, check_is_str)
        sn_check(theme, check_is_str)
        box = self.box(**box_args)
        box._content = TextContent(
            text=text,
            style=style,
            align=align,
            is_code=True,
            syntax_language=language,
            syntax_theme=theme,
            parse_styles=parse_styles,
            style_delimiters=style_delimiters,
        )
        return box

    def image(
        self,
        path_or_data: Sn[PathOrImageData],
        *,
        enable_steps: Sv[bool] = True,
        shift_steps: int = 0,
        **box_args,
    ):
        sn_check(path_or_data, check_image_path_or_data)
        sv_check(enable_steps, check_is_bool)
        sv_check(shift_steps, check_is_int)
        path_or_data = sn_map(path_or_data, normalize_and_watch_image_path)
        box = self.box(**box_args)
        box._content = ImageContent(path_or_data, enable_steps, shift_steps)
        return box

    def set_style(self, name: str, style: Sn[TextStyle]):
        if name == "default":
            self.update_style(name, style)
            return
        check_is_str(name)
        sn_check(style, check_is_text_style)
        self._set_style(name, style)

    def update_style(self, name: str, style: TextStyle):
        check_is_str(name)
        check_is_text_style(style)
        old_style = self._get_style(name)
        if old_style is None:
            self._set_style(name, style)
        elif isinstance(old_style, TextStyle):
            self._set_style(name, old_style.merge(style))
        else:
            raise Exception(
                "Non-primitive style cannot be updated; use set_style instead"
            )

    def x(self, width_fraction: IntOrFloat = 0) -> LayoutExpr:
        """
        Get an expression with X coordinate relative to the box.
        """
        check_is_int_or_float(width_fraction)
        node_id = id(self)
        expr = LayoutExpr.x(node_id)
        if width_fraction == 0:
            return expr
        return expr + LayoutExpr.width(node_id, width_fraction)

    def y(self, height_fraction: IntOrFloat = 0) -> LayoutExpr:
        """
        Get an expression with Y coordinate relative to the box.
        """
        check_is_int_or_float(height_fraction)
        node_id = id(self)
        expr = LayoutExpr.y(node_id)
        if height_fraction == 0:
            return expr
        return expr + LayoutExpr.height(node_id, height_fraction)

    def p(self, x: IntOrFloat = 0, y: IntOrFloat = 0) -> Point:
        """
        Get an expression with X and Y padding relative to the box.
        """
        return Point(self.x(x), self.y(y))

    def width(self, fraction: IntOrFloat = 1.0) -> LayoutExpr:
        """
        Get an expression with width of the parent box.
        """
        check_is_int_or_float(fraction)
        node_id = id(self)
        return LayoutExpr.width(node_id, fraction)

    def height(self, fraction: IntOrFloat = 1.0) -> LayoutExpr:
        """
        Get an expression with height of the parent box.
        """
        check_is_int_or_float(fraction)
        node_id = id(self)
        return LayoutExpr.height(node_id, fraction)

    def line_x(self, line_idx: int, width_fraction: IntOrFloat = 0) -> LayoutExpr:
        """
        Get an expression with X coordinate of a given line of text in the box.
        """
        check_is_int_or_float(width_fraction)
        check_is_int(line_idx)
        node_id = id(self)
        expr = LayoutExpr.line_x(node_id, line_idx)
        if width_fraction == 0:
            return expr
        return expr + LayoutExpr.line_width(node_id, line_idx, width_fraction)

    def line_y(self, line_idx: int, height_fraction: IntOrFloat = 0) -> LayoutExpr:
        """
        Get an expression with Y coordinate of a given line of text in the box.
        """
        check_is_int_or_float(height_fraction)
        check_is_int(line_idx)
        node_id = id(self)
        expr = LayoutExpr.line_y(node_id, line_idx)
        if height_fraction == 0:
            return expr
        return expr + LayoutExpr.line_height(node_id, line_idx, height_fraction)

    def line_p(self, line_idx: int, x: IntOrFloat = 0, y: IntOrFloat = 0) -> Point:
        return Point(self.line_x(line_idx, x), self.line_y(line_idx, y))

    def line_width(self, line_idx: int, fraction: IntOrFloat = 1.0) -> LayoutExpr:
        """
        Get an expression with a width of a given line of text in the box.
        """
        check_is_int_or_float(fraction)
        check_is_int(line_idx)
        node_id = id(self)
        return LayoutExpr.line_width(node_id, line_idx, fraction)

    def line_height(self, line_idx: int, fraction: IntOrFloat = 1.0) -> LayoutExpr:
        """
        Get an expression with a height of a given line of text in the box.
        """
        check_is_int_or_float(fraction)
        check_is_int(line_idx)
        node_id = id(self)
        return LayoutExpr.line_height(node_id, line_idx, fraction)

    def inline_x(self, anchor_id: int, width_fraction: IntOrFloat = 0) -> LayoutExpr:
        """
        Get an expression with X coordinate of a given text anchor in the box.
        """
        check_is_int_or_float(width_fraction)
        check_is_int(anchor_id)
        node_id = id(self)
        expr = LayoutExpr.inline_x(node_id, anchor_id)
        if width_fraction == 0:
            return expr
        return expr + LayoutExpr.inline_width(node_id, anchor_id, width_fraction)

    def inline_y(self, anchor_id: int, height_fraction: IntOrFloat = 0) -> LayoutExpr:
        """
        Get an expression with Y coordinate of a given text anchor in the box.
        """
        check_is_int_or_float(height_fraction)
        check_is_int(anchor_id)
        node_id = id(self)
        expr = LayoutExpr.inline_y(node_id, anchor_id)
        if height_fraction == 0:
            return expr
        return expr + LayoutExpr.inline_height(node_id, anchor_id, height_fraction)

    def inline_p(self, anchor_id: int, x: IntOrFloat = 0, y: IntOrFloat = 0) -> Point:
        return Point(self.inline_x(anchor_id, x), self.inline_y(anchor_id, y))

    def inline_width(self, anchor_id: int, fraction: IntOrFloat = 1.0) -> LayoutExpr:
        """
        Get an expression with a height of a given text anchor in the box.
        """
        check_is_int_or_float(fraction)
        check_is_int(anchor_id)
        node_id = id(self)
        return LayoutExpr.inline_width(node_id, anchor_id, fraction)

    def inline_height(self, anchor_id: int, fraction: IntOrFloat = 1.0) -> LayoutExpr:
        """
        Get an expression with a height of a given text anchor in the box.
        """
        check_is_int_or_float(fraction)
        check_is_int(anchor_id)
        node_id = id(self)
        return LayoutExpr.inline_height(node_id, anchor_id, fraction)

    def line_box(self, line_idx: int, n_lines: int = 1, **box_args):
        """
        Creates a new box over a text line in the box
        """
        height = self.line_height(line_idx)
        if n_lines != 1:
            check_is_int(line_idx)
            height = height * n_lines
            width = LayoutExpr.max(
                [self.line_width(line_idx + i) for i in range(n_lines)]
            )
        else:
            width = self.line_width(line_idx)
        return self.box(
            x=self.line_x(line_idx),
            y=self.line_y(line_idx),
            width=width,
            height=height,
            **box_args,
        )

    def inline_box(self, anchor_id: int, **box_args):
        """
        Creates a new box over a inline text anchor in the box
        """

        return self.box(
            x=self.inline_x(anchor_id),
            y=self.inline_y(anchor_id),
            width=self.inline_width(anchor_id),
            height=self.inline_height(anchor_id),
            **box_args,
        )


class Box(BoxBuilderMixin):
    def __init__(
        self,
        *,
        x: Sn[Position] = None,
        y: Sn[Position] = None,
        show: BoolStepDef = True,
        active: BoolStepDef = True,
        z_level: Sn[int] = None,
        width: Sn[Size] = None,
        height: Sn[Size] = None,
        bg_color: Sn[str] = None,
        row: Sv[bool] = False,
        reverse: Sv[bool] = False,
        p_left: Sv[Length] = 0,
        p_right: Sv[Length] = 0,
        p_top: Sv[Length] = 0,
        p_bottom: Sv[Length] = 0,
        m_left: Sv[LengthAuto] = 0,
        m_right: Sv[LengthAuto] = 0,
        m_top: Sv[LengthAuto] = 0,
        m_bottom: Sv[LengthAuto] = 0,
        flex_grow: Sv[float] = 0.0,
        flex_shrink: Sv[float] = 1.0,
        align_items: Sn[AlignItems] = None,
        align_self: Sn[AlignItems] = None,
        justify_self: Sn[AlignItems] = None,
        align_content: Sn[AlignContent] = None,
        justify_content: Sn[AlignContent] = None,
        gap_x: Sv[Length] = 0,
        gap_y: Sv[Length] = 0,
        grid: Sn[GridOptions] = None,
        border_radius: Sv[IntOrFloat] = 0,
        url: Sn[str] = None,
        name: str = "",
        debug_layout: bool | str | None = None,
    ):
        """
        Create a new box. See [Box reference](https://spirali.github.io/nelsie/guide/box/) for documentation.
        """
        sn_check(x, check_position)
        sn_check(y, check_position)
        sn_check(z_level, check_is_int)
        sn_check(width, check_size)
        sn_check(height, check_size)
        sn_check(bg_color, check_color)
        sv_check(row, check_is_bool)
        sv_check(reverse, check_is_bool)
        sv_check(p_left, check_length)
        sv_check(p_right, check_length)
        sv_check(p_top, check_length)
        sv_check(p_bottom, check_length)
        sv_check(m_left, check_length_auto)
        sv_check(m_right, check_length_auto)
        sv_check(m_top, check_length_auto)
        sv_check(m_bottom, check_length_auto)

        sv_check(flex_grow, check_is_int_or_float)
        sv_check(flex_shrink, check_is_int_or_float)
        sv_check(gap_x, check_length)
        sv_check(gap_y, check_length)
        sn_check(align_items, check_align_items)
        sn_check(align_self, check_align_items)
        sn_check(justify_self, check_align_items)
        sn_check(align_content, check_align_content)
        sn_check(justify_content, check_align_content)
        sv_check(border_radius, check_is_int_or_float)
        sn_check(url, check_is_str)
        check_is_str(name)

        if isinstance(debug_layout, str):
            check_color(debug_layout)

        self._show = parse_bool_steps(show)
        self._active = parse_bool_steps(active)
        self._x = x
        self._y = y
        self._z_level = z_level
        self._width = width
        self._height = height
        self._bg_color = bg_color
        self._content = None
        self._children = []
        self._row = row
        self._reverse = reverse
        self._p_left = p_left
        self._p_right = p_right
        self._p_top = p_top
        self._p_bottom = p_bottom
        self._m_left = m_left
        self._m_right = m_right
        self._m_top = m_top
        self._m_bottom = m_bottom

        self._flex_grow = flex_grow
        self._flex_shrink = flex_shrink
        self._align_items = align_items
        self._align_self = align_self
        self._justify_self = justify_self
        self._align_content = align_content
        self._justify_content = justify_content
        self._gap_x = gap_x
        self._gap_y = gap_y
        self._grid = grid
        self._debug_layout = debug_layout
        self._border_radius = border_radius
        self._url = url
        self.name = name
        self._text_styles: dict[str, Sn[TextStyle]] | None = None

    def margin(
        self,
        all: Sn[LengthAuto] = None,
        *,
        x: Sn[LengthAuto] = None,
        y: Sn[LengthAuto] = None,
        left: Sn[LengthAuto] = None,
        right: Sn[LengthAuto] = None,
        top: Sn[LengthAuto] = None,
        bottom: Sn[LengthAuto] = None,
    ):
        """
        Sets box's margin
        """
        if all is not None:
            sn_check(all, check_length_auto)
            self._m_top = all
            self._m_bottom = all
            self._m_left = all
            self._m_right = all

        if x is not None:
            sn_check(x, check_length_auto)
            self._m_left = x
            self._m_right = x

        if y is not None:
            sn_check(y, check_length_auto)
            self._m_top = y
            self._m_bottom = y

        if left is not None:
            sn_check(left, check_length_auto)
            self._m_left = left

        if right is not None:
            sn_check(right, check_length_auto)
            self._m_right = right

        if top is not None:
            sn_check(top, check_length_auto)
            self._m_top = top

        if bottom is not None:
            sn_check(bottom, check_length_auto)
            self._m_bottom = bottom
        return self

    def padding(
        self,
        all: Sn[LengthAuto] = None,
        *,
        x: Sn[LengthAuto] = None,
        y: Sn[LengthAuto] = None,
        left: Sn[LengthAuto] = None,
        right: Sn[LengthAuto] = None,
        top: Sn[LengthAuto] = None,
        bottom: Sn[LengthAuto] = None,
    ):
        """
        Sets box's padding.
        """
        if all is not None:
            sn_check(all, check_length)
            self._p_top = all
            self._p_bottom = all
            self._p_left = all
            self._p_right = all

        if x is not None:
            sn_check(x, check_length)
            self._p_left = x
            self._p_right = x

        if y is not None:
            sn_check(y, check_length)
            self._p_top = y
            self._p_bottom = y

        if left is not None:
            sn_check(left, check_length)
            self._p_left = left

        if right is not None:
            sn_check(right, check_length)
            self._p_right = right

        if top is not None:
            sn_check(top, check_length)
            self._p_top = top

        if bottom is not None:
            sn_check(bottom, check_length)
            self._p_bottom = bottom
        return self

    def draw_line(self, p1: Point, p2: Point, **path_args):
        """
        Shortcut for drawing a simple line
        """
        path = Path(**path_args)
        path.move_to(p1)
        path.line_to(p2)
        return self.add(path)

    def add(self, item):
        """
        Adds Box or a geometry item into the box
        """
        self._children.append(item)

    def traverse_tree(self, shared_data, steps):
        if self._content is not None:
            self._content.traverse_tree(shared_data, steps)
        traverse_children(self._children, shared_data, steps)

    def _set_style(self, name: str, style: Sn[TextStyle]):
        if self._text_styles is None:
            self._text_styles = {}
        self._text_styles[name] = style

    def _get_style(self, name: str) -> Sn[TextStyle] | None:
        if self._text_styles is None:
            return None
        return self._text_styles.get(name)


def traverse_children(children, shared_data, steps):
    for child in children:
        if isinstance(child, Box):
            child.traverse_tree(shared_data, steps)
        elif isinstance(child, StepVal):
            sn_apply(
                child,
                lambda c: c.traverse_tree(shared_data, steps)
                if isinstance(c, Box)
                else None,
            )
