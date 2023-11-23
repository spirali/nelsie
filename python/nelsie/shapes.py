from dataclasses import dataclass

from .layoutexpr import LayoutExpr
from .steps.insteps import InSteps

PathValue = LayoutExpr | InSteps[LayoutExpr]


@dataclass
class Move:
    x: PathValue
    y: PathValue


@dataclass
class Line:
    x: PathValue
    y: PathValue


class Path:
    def __init__(self):
        self.parts = []

    def move_to(self, x: PathValue, y: PathValue):
        self.parts.append(Move(x, y))
        return self

    def line_to(self, x: PathValue, y: PathValue):
        self.parts.append(Line(x, y))
        return self
