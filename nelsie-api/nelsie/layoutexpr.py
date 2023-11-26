from dataclasses import dataclass


class LayoutExpr:
    def __add__(self, other: "LayoutExpr"):
        return SumExpr([self, other])


@dataclass
class ConstExpr(LayoutExpr):
    _tag = "const_value"
    value: float


@dataclass
class XExpr(LayoutExpr):
    _tag = "x"
    node_id: int


@dataclass
class YExpr(LayoutExpr):
    _tag = "y"
    node_id: int


@dataclass
class SumExpr(LayoutExpr):
    _tag = "sum"
    expressions: list[LayoutExpr]

    def __add__(self, other: LayoutExpr):
        expressions = self.expressions[:]
        expressions.append(other)
        return SumExpr(expressions)
