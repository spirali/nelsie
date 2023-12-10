from .basictypes import Stroke
from .insteps import InSteps
from .layoutexpr import LayoutExpr

PathValue = int | float | LayoutExpr | InSteps[LayoutExpr]


class Path:
    def __init__(self, stroke: None | Stroke = None):
        self.stroke = stroke
        self.commands = []
        self.points = []

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
