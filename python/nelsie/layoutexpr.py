class LayoutExpr:
    def __init__(self, inner_expr):
        self._expr = inner_expr

    def __add__(self, other):
        other = unpack_layout_expr_arg(other)
        if self._expr[0] == "sum":
            expr = self._expr + (other,)
        else:
            expr = ("sum", self._expr, other)
        return LayoutExpr(expr)

    def __sub__(self, other):
        if isinstance(other, float) or isinstance(other, int):
            return self + (-other)
        raise Exception("TODO")

    @staticmethod
    def x(node_id):
        return LayoutExpr(("x", node_id))

    @staticmethod
    def y(node_id):
        return LayoutExpr(("y", node_id))

    @staticmethod
    def width(node_id, fraction):
        return LayoutExpr(("width", node_id, fraction))

    @staticmethod
    def height(node_id, fraction):
        return LayoutExpr(("height", node_id, fraction))


def unpack_layout_expr_arg(obj):
    if isinstance(obj, LayoutExpr):
        return obj._expr
    if isinstance(obj, float) or isinstance(obj, int):
        return obj
    else:
        raise Exception("Invalid parameter for layout expression")
