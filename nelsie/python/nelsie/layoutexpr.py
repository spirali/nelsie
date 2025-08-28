from typing import Union, Sequence

from .utils import check_is_type

IntOrFloatOrLayoutExpr = Union[int, float, "LayoutExpr"]


class LayoutExpr:
    def __init__(self, op, arg0=None, arg1=None, arg2=None):
        self._op = op
        self._arg0 = arg0
        self._arg1 = arg1
        self._arg2 = arg2

    def __add__(self, other: IntOrFloatOrLayoutExpr):
        check_int_or_float_or_layout_expr(other)
        return LayoutExpr("+", self, other)

    def __mul__(self, other: IntOrFloatOrLayoutExpr):
        check_int_or_float_or_layout_expr(other)
        return LayoutExpr("*", self, other)

    def __sub__(self, other: IntOrFloatOrLayoutExpr):
        check_int_or_float_or_layout_expr(other)
        return LayoutExpr("-", self, other)

    @staticmethod
    def max(expressions: Sequence["LayoutExpr"]):
        return LayoutExpr("max", expressions)

    @staticmethod
    def x(node_id):
        return LayoutExpr("x", node_id)

    @staticmethod
    def y(node_id):
        return LayoutExpr("y", node_id)

    @staticmethod
    def width(node_id, fraction):
        return LayoutExpr("width", node_id, fraction)

    @staticmethod
    def height(node_id, fraction):
        return LayoutExpr("height", node_id, fraction)

    @staticmethod
    def line_x(node_id, line_idx):
        return LayoutExpr("line_x", node_id, line_idx)

    @staticmethod
    def line_y(node_id, line_idx):
        return LayoutExpr("line_y", node_id, line_idx)

    @staticmethod
    def line_width(node_id, line_idx, fraction):
        return LayoutExpr("line_width", node_id, line_idx, fraction)

    @staticmethod
    def line_height(node_id, line_idx, fraction):
        return LayoutExpr("line_height", node_id, line_idx, fraction)

    @staticmethod
    def inline_x(node_id, anchor_id):
        return LayoutExpr("inline_x", node_id, anchor_id)

    @staticmethod
    def inline_y(node_id, anchor_id):
        return LayoutExpr("inline_y", node_id, anchor_id)

    @staticmethod
    def inline_width(node_id, anchor_id, fraction):
        return LayoutExpr("inline_width", node_id, anchor_id, fraction)

    @staticmethod
    def inline_height(node_id, anchor_id, fraction):
        return LayoutExpr("inline_height", node_id, anchor_id, fraction)


int_or_float_or_layout_expr = (int, float, LayoutExpr)


def check_int_or_float_or_layout_expr(obj):
    check_is_type(obj, int_or_float_or_layout_expr)
