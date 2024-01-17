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

    @staticmethod
    def line_x(node_id, line_idx):
        return LayoutExpr(("line_x", node_id, line_idx))

    @staticmethod
    def line_y(node_id, line_idx):
        return LayoutExpr(("line_y", node_id, line_idx))

    @staticmethod
    def line_width(node_id, line_idx, fraction):
        return LayoutExpr(("line_width", node_id, line_idx, fraction))

    @staticmethod
    def line_height(node_id, line_idx, fraction):
        return LayoutExpr(("line_height", node_id, line_idx, fraction))

    @staticmethod
    def text_anchor_x(node_id, anchor_id):
        return LayoutExpr(("anchor_x", node_id, anchor_id))

    @staticmethod
    def text_anchor_y(node_id, anchor_id):
        return LayoutExpr(("anchor_y", node_id, anchor_id))

    @staticmethod
    def text_anchor_width(node_id, anchor_id, fraction):
        return LayoutExpr(("anchor_width", node_id, anchor_id, fraction))

    @staticmethod
    def text_anchor_height(node_id, anchor_id, fraction):
        return LayoutExpr(("anchor_height", node_id, anchor_id, fraction))


def unpack_layout_expr_arg(obj):
    if isinstance(obj, LayoutExpr):
        return obj._expr
    if isinstance(obj, float) or isinstance(obj, int):
        return obj
    else:
        raise Exception("Invalid parameter for layout expression")
