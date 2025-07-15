from dataclasses import dataclass
from copy import copy


from .image import PathOrImageData, ImageContent, check_image_path_or_data, normalize_image_path
from .steps import Sn, Step, get_step, Sv, sv_check, sn_check, parse_bool_steps, sn_map, BoolStepDef
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
)
from .nelsie import check_color
from .text import TextContent
from .textstyle import TextStyle, merge_in_step, check_is_text_style, check_is_str_or_text_style
from .utils import check_is_str, check_is_bool, check_is_int, check_is_int_or_float


class BoxBuilderMixin:
    def add(self, box: "Box"):
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
    ):
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
        )
        self.add(box)
        return box

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
        self, path_or_data: Sn[PathOrImageData], *, enable_steps: Sv[bool] = True, shift_steps: Sv[int] = 0, **box_args
    ):
        sn_check(path_or_data, check_image_path_or_data)
        sv_check(enable_steps, check_is_bool)
        sv_check(shift_steps, check_is_int)
        path_or_data = sn_map(path_or_data, normalize_image_path)
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
            raise Exception("Non-primitive style cannot be updated; use set_style instead")


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
    ):
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

        self._text_styles: dict[str, Sn[TextStyle]] | None = None

    def x(self, x: Sn[Position]):
        check_position(x)
        self._x = x

    def y(self, y: Sn[Position]):
        check_position(y)
        self._y = y

    def z_level(self, z_level: Sn[int]):
        check_is_int(z_level)
        self._z_level = z_level

    def row(self, row: Sv[bool]):
        sv_check(row, check_is_bool)
        self._row = row

    def width(self, width: Sn[Size]):
        check_size(width)
        self._width = width

    def height(self, height: Sn[Size]):
        check_size(height)
        self._height = height

    def bg_color(self, bg_color: Sn[str]):
        if bg_color:
            check_color(bg_color)
        self._bg_color = bg_color

    def add(self, box: "Box"):
        self._children.append(box)

    def traverse_tree(self, shared_data, steps):
        for child in self._children:
            child.traverse_tree(shared_data, steps)
        if self._content is not None:
            self._content.traverse_tree(shared_data, steps)

    def _set_style(self, name: str, style: Sn[TextStyle]):
        if self._text_styles is None:
            self._text_styles = {}
        self._text_styles[name] = style

    def _get_style(self, name: str) -> Sn[TextStyle] | None:
        if self._text_styles is None:
            return None
        return self._text_styles.get(name)
