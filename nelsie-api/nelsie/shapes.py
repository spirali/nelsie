from dataclasses import dataclass

from .basictypes import Stroke
from .export import (
    ExportPath,
    ExportPathCubic,
    ExportPathLine,
    ExportPathMove,
    ExportPathQuad,
)
from .layoutexpr import ConstExpr, LayoutExpr, XExpr, YExpr
from .steps.insteps import InSteps

PathValue = int | float | LayoutExpr | InSteps[LayoutExpr]


def parse_point_position(parent_id, value, is_x):
    if isinstance(value, LayoutExpr):
        return value
    if isinstance(value, (int, float)):
        if is_x:
            return XExpr(parent_id) + ConstExpr(value)
        else:
            return YExpr(parent_id) + ConstExpr(value)
    raise Exception("Invalid position for point")


@dataclass
class Move:
    x: PathValue
    y: PathValue

    def export(self, parent_id: int):
        return ExportPathMove(
            x=parse_point_position(parent_id, self.x, True),
            y=parse_point_position(parent_id, self.y, False),
        )


@dataclass
class Line:
    x: PathValue
    y: PathValue

    def export(self, parent_id: int):
        return ExportPathLine(
            x=parse_point_position(parent_id, self.x, True),
            y=parse_point_position(parent_id, self.y, False),
        )


@dataclass
class Quad:
    x1: PathValue
    y1: PathValue
    x: PathValue
    y: PathValue

    def export(self, parent_id: int):
        return ExportPathQuad(
            x1=parse_point_position(parent_id, self.x1, True),
            y1=parse_point_position(parent_id, self.y1, False),
            x=parse_point_position(parent_id, self.x, True),
            y=parse_point_position(parent_id, self.y, False),
        )


@dataclass
class Cubic:
    x1: PathValue
    y1: PathValue
    x2: PathValue
    y2: PathValue
    x: PathValue
    y: PathValue

    def export(self, parent_id: int):
        return ExportPathCubic(
            x1=parse_point_position(parent_id, self.x1, True),
            y1=parse_point_position(parent_id, self.y1, False),
            x2=parse_point_position(parent_id, self.x2, True),
            y2=parse_point_position(parent_id, self.y2, False),
            x=parse_point_position(parent_id, self.x, True),
            y=parse_point_position(parent_id, self.y, False),
        )


class Path:
    def __init__(self, stroke: None | Stroke = None):
        self.stroke = stroke
        self.parts = []

    def export(self, parent_id: int):
        return ExportPath(
            stroke=self.stroke, parts=[part.export(parent_id) for part in self.parts]
        )

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
