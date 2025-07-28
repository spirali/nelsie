from dataclasses import dataclass
from copy import copy

from .doc import RawBox
from .image import PathOrImageData, ImageContent, check_image_path_or_data
from .steps import Sn, Step, get_step, Sv, sv_check, sn_check
from .basictypes import Position, Size, check_position, check_size, TextAlign, check_text_align
from .nelsie import check_color
from .text import TextContent
from .textstyle import TextStyle, merge_in_step, check_is_text_style
from .utils import check_is_str, check_is_bool, check_is_int


class BoxBuilderMixin:
    def add(self, box: "Box"):
        raise NotImplementedError

    def box(
            self,
            *,
            x: Sn[Position] = None,
            y: Sn[Position] = None,
            width: Sn[Size] = None,
            height: Sn[Size] = None,
            bg_color: Sn[str] = None,
    ):
        box = Box(x=x, y=y, width=width, height=height, bg_color=bg_color)
        self.add(box)
        return box

    def text(self, text: Sv[str], style: Sn[TextStyle] = None, align: Sv[TextAlign] = "start", strip: bool = True,
             **box_args):
        if strip and isinstance(text, str):
            text = text.strip()
        sv_check(text, check_is_str)
        sv_check(align, check_text_align)
        sn_check(style, check_is_text_style)
        box = self.box(**box_args)
        box._content = TextContent(text, style, align, None, None)

    def image(self, path_or_data: Sn[PathOrImageData],
              *,
              enable_steps: Sv[bool] = True,
              shift_steps: Sv[int] = 0,
              **box_args):
        sn_check(path_or_data, check_image_path_or_data)
        sv_check(enable_steps, check_is_bool)
        sv_check(shift_steps, check_is_int)
        box = self.box(**box_args)
        box._content = ImageContent(path_or_data, enable_steps, shift_steps)


class Box(BoxBuilderMixin):
    def __init__(
            self,
            *,
            x: Sn[Position] = None,
            y: Sn[Position] = None,
            width: Sn[Size] = None,
            height: Sn[Size] = None,
            bg_color: Sn[str] = None,
    ):
        if x:
            check_position(x)
        if y:
            check_position(y)
        if width:
            check_size(width)
        if height:
            check_size(height)
        if bg_color:
            check_color(bg_color)

        self._x = x
        self._y = y
        self._width = width
        self._height = height
        self._bg_color = bg_color
        self._content = None
        self._children = []

        self._text_style: Sn[TextStyle] = None
        self._code_style: Sn[TextStyle] = None

    def x(self, x: Sn[Position]):
        check_position(x)
        self._x = x

    def y(self, y: Sn[Position]):
        check_position(y)
        self._y = y

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

    def traverse_tree(self, shared_data):
        for child in self._children:
            child.traverse_children(shared_data)
        if self._content is not None:
            self._content.traverse_tree(shared_data)
