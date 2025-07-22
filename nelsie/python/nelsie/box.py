from .doc import RawBox
from .steps import Sv, Step, at_step
from nelsie.basictypes import Position, Size, check_position, check_size
from .nelsie import check_color


class BoxBuilderMixin:
    def add(self, box: "Box"):
        raise NotImplementedError

    def box(
        self,
        *,
        x: Sv[Position] = None,
        y: Sv[Position] = None,
        width: Sv[Size] = None,
        height: Sv[Size] = None,
        bg_color: Sv[str | None] = None,
    ):
        box = Box(x=x, y=y, width=width, height=height, bg_color=bg_color)
        self.add(box)
        return box


class Box(BoxBuilderMixin):
    def __init__(
        self,
        *,
        x: Sv[Position] = None,
        y: Sv[Position] = None,
        width: Sv[Size] = None,
        height: Sv[Size] = None,
        bg_color: Sv[str | None] = None,
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

    def x(self, x: Sv[Position]):
        check_position(x)
        self._x = x

    def y(self, y: Sv[Position]):
        check_position(y)
        self._y = y

    def width(self, width: Sv[Size]):
        check_size(width)
        self._width = width

    def height(self, height: Sv[Size]):
        check_size(height)
        self._height = height

    def bg_color(self, bg_color: Sv[str]):
        if bg_color:
            check_color(bg_color)
        self._bg_color = bg_color

    def add(self, box: "Box"):
        self._children.append(box)

    def at_step(self, step: Step) -> RawBox:
        return RawBox(
            x=at_step(self._x, step),
            y=at_step(self._y, step),
            width=at_step(self._width, step),
            height=at_step(self._height, step),
            bg_color=at_step(self._bg_color, step),
            children=[child.at_step(step) for child in self._children],
        )
