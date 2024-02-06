from dataclasses import dataclass

from .basictypes import Stroke
from .layoutexpr import LayoutExpr

PathValue = int | float | LayoutExpr


@dataclass
class Arrow:
    """
    Represents an SVG arrow head. Can be attached to the start or end points of lines.
    """

    size: float = 10
    """Size of the arrow head in pixels."""

    angle: float = 40
    """Angle of the arrow head."""

    color: str | None = None
    """Color of arrow, if None color is taken from path"""

    stroke_width: float | None = None
    """If None then a filled arrow is drawn, if float then stroked arrow is drawn with the given stroke width"""

    inner_point: float | None = None
    """ Shape of the arrow head.

        * < 1.0 -> Sharper arrow.
        * = 1.0 -> Normal arrow.
        * > 1.0 -> Diamond shape arrow.
    """


class Path:
    def __init__(
        self,
        *,
        stroke: Stroke | None = None,
        fill_color: str | None = None,
        arrow_start: Arrow | None = None,
        arrow_end: Arrow | None = None,
    ):
        self.stroke = stroke
        self.fill_color = fill_color
        self.commands = []
        self.points = []
        self.arrow_start = arrow_start
        self.arrow_end = arrow_end

    @staticmethod
    def oval(
        x1: PathValue,
        y1: PathValue,
        x2: PathValue,
        y2: PathValue,
        *,
        stroke: Stroke | None = None,
        fill_color: str | None = None,
    ):
        path = Path(stroke=stroke, fill_color=fill_color)
        path.commands.append("oval")
        path.points = [x1, y1, x2, y2]
        return path

    def close(self):
        self.commands.append("close")
        return self

    def move_to(self, x: PathValue, y: PathValue):
        self.commands.append("move")
        self.points.append(x)
        self.points.append(y)
        return self

    def line_to(self, x: PathValue, y: PathValue):
        self.commands.append("line")
        self.points.append(x)
        self.points.append(y)
        return self

    def quad_to(self, x1: PathValue, y1: PathValue, x: PathValue, y: PathValue):
        self.commands.append("quad")
        self.points.append(x1)
        self.points.append(y1)
        self.points.append(x)
        self.points.append(y)
        return self

    def cubic_to(
        self,
        x1: PathValue,
        y1: PathValue,
        x2: PathValue,
        y2: PathValue,
        x: PathValue,
        y: PathValue,
    ):
        self.commands.append("cubic")
        self.points.append(x1)
        self.points.append(y1)
        self.points.append(x2)
        self.points.append(y2)
        self.points.append(x)
        self.points.append(y)
        return self
