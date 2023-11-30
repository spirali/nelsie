from dataclasses import dataclass


class LayoutExpr:
    def __add__(self, other: "LayoutExpr"):
        return SumExpr([self, other])


@dataclass
class ConstExpr(LayoutExpr):
    value: float


@dataclass
class XExpr(LayoutExpr):
    node_id: int


@dataclass
class YExpr(LayoutExpr):
    node_id: int


@dataclass
class SumExpr(LayoutExpr):
    expressions: list[LayoutExpr]

    def __add__(self, other: LayoutExpr):
        expressions = self.expressions[:]
        expressions.append(other)
        return SumExpr(expressions)
