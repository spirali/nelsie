from dataclasses import dataclass

from .basictypes import Stroke
from .insteps import InSteps
from .layoutexpr import LayoutExpr

PathValue = int | float | LayoutExpr | InSteps[LayoutExpr]


@dataclass
class Arrow:
    """
    Represents an SVG arrow head.

    Can be attached to the start or end points of lines.

    Attributes
    ----------
    size : float
        Size of the arrow head in pixels.
    angle : float
        Angle of the arrow head.
    color: str | None
        Color of arrow, if None color is taken from path
    stroke_width : float | None
        Width of the arrow head edge.
    inner_point : float
        Shape of the arrow head.
        < 1.0 -> Sharper arrow.
        = 1.0 -> Normal arrow.
        > 1.0 -> Diamond shape arrow.
    """

    size: float = 10
    angle: float = 40
    color: str | None = None
    stroke_width: float | None = None
    inner_point: float | None = None


class Path:
    def __init__(
        self,
        stroke: None | Stroke = None,
        arrow_start: Arrow | None = None,
        arrow_end: Arrow | None = None,
    ):
        self.stroke = stroke
        self.commands = []
        self.points = []
        self.arrow_start = arrow_start
        self.arrow_end = arrow_end

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
