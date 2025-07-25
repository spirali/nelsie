from .doc import RawBox
from .steps import Sn, Step, get_step
from .basictypes import Position, Size, check_position, check_size
from .nelsie import check_color
from .text import TextContent
from .textstyle import TextStyle


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

        self.text_style = None
        self.code_style = None

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

    def text(self, text: Sn[str], style: Sn[TextStyle],  **box_args):
        box = self.box(**box_args)
        box._content = TextContent(text, style)
    def add(self, box: "Box"):
        self._children.append(box)

    def at_step(self, step: Step) -> RawBox:
        return RawBox(
            x=get_step(self._x, step),
            y=get_step(self._y, step),
            width=get_step(self._width, step),
            height=get_step(self._height, step),
            bg_color=get_step(self._bg_color, step),
            children=[child.get_step(step) for child in self._children],
        )
