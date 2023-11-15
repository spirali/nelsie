from dataclasses import dataclass


class LayoutBaseExpr:
    pass


@dataclass
class ConstExpr(LayoutBaseExpr):
    _tag = "const_value"
    value: float
