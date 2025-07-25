from dataclasses import dataclass
from copy import copy
from .doc import RawBox
from .steps import Sn, Step, get_step, Sv
from .basictypes import Position, Size, check_position, check_size, TextAlign
from .nelsie import check_color
from .text import TextContent
from .textstyle import TextStyle, merge_in_step


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

    def text(self, text: Sn[str], style: Sn[TextStyle] = None, align: Sv[TextAlign] = "left", **box_args):
        box = self.box(**box_args)
        box._content = TextContent(text, style, align, None, None)


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
