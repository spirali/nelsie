from dataclasses import dataclass


class LayoutBaseExpr:
    def __add__(self, other: "LayoutBaseExpr"):
        return SumExpr([self, other])


@dataclass
class ConstExpr(LayoutBaseExpr):
    _tag = "const_value"
    value: float


@dataclass
class XExpr(LayoutBaseExpr):
    _tag = "x"
    node_id: int


@dataclass
class YExpr(LayoutBaseExpr):
    _tag = "y"
    node_id: int


@dataclass
class SumExpr(LayoutBaseExpr):
    _tag = "sum"
    expressions: list[LayoutBaseExpr]

    def __add__(self, other: LayoutBaseExpr):
        expressions = self.expressions[:]
        expressions.append(other)
        return SumExpr(expressions)
