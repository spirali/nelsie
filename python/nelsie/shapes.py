from dataclasses import dataclass

from .basictypes import Stroke
from .insteps import InSteps
from .layoutexpr import LayoutExpr


PathValue = int | float | LayoutExpr | InSteps[LayoutExpr]


@dataclass
class Move:
    x: PathValue
    y: PathValue


@dataclass
class Line:
    x: PathValue
    y: PathValue


@dataclass
class Quad:
    x1: PathValue
    y1: PathValue
    x: PathValue
    y: PathValue


@dataclass
class Cubic:
    x1: PathValue
    y1: PathValue
    x2: PathValue
    y2: PathValue
    x: PathValue
    y: PathValue


class Path:
    def __init__(self, stroke: None | Stroke = None):
        self.stroke = stroke
        self.parts = []

    def move_to(self, x: PathValue, y: PathValue):
        self.parts.append(Move(x, y))
        return self

    def line_to(self, x: PathValue, y: PathValue):
        self.parts.append(Line(x, y))
        return self

    def quad_to(self, x1: PathValue, y1: PathValue, x: PathValue, y: PathValue):
        self.parts.append(Quad(x1, y1, x, y))
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
        self.parts.append(Cubic(x1, y1, x2, y2, x, y))
        return self

    def copy(self):
        path = Path(self.stroke)
        path.parts = self.parts[:]
        return path
