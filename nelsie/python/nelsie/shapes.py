from dataclasses import dataclass

from nelsie.nelsie import check_color

from .layoutexpr import LayoutExpr, IntOrFloatOrLayoutExpr, check_int_or_float_or_layout_expr
from .steps import Sv, Sn, get_step, Step, sv_check, sn_check, BoolStepDef, parse_bool_steps
from .utils import check_is_int_or_float, check_is_type, check_is_int

PathValue = int | float | LayoutExpr


@dataclass(frozen=True)
class Point:
    x: Sv[IntOrFloatOrLayoutExpr]
    y: Sv[IntOrFloatOrLayoutExpr]

    def get_step(self, step: Step):
        return Point(get_step(self.x, step), get_step(self.y, step))

    def __post_init__(self):
        sv_check(self.x, check_int_or_float_or_layout_expr)
        sv_check(self.y, check_int_or_float_or_layout_expr)


def check_is_point(obj):
    check_is_type(obj, Point)


@dataclass(frozen=True)
class Stroke:
    color: Sv[str]
    width: Sv[float] = 1.0
    dash_array: Sn[list[float]] = None
    dash_offset: Sv[float] = 0.0

    def to_raw(self, step: Step):
        return Stroke(
            get_step(self.color, step),
            get_step(self.width, step),
            get_step(self.dash_array, step),
            get_step(self.dash_offset, step),
        )

    def __post_init__(self):
        sv_check(self.color, check_color)
        sv_check(self.width, check_is_int_or_float)
        sv_check(self.dash_offset, check_is_int_or_float)


def check_is_stroke(obj):
    check_is_type(obj, Stroke)


@dataclass(frozen=True)
class RawRect:
    shape: int  # 0 = rect, 1 = oval
    x1: IntOrFloatOrLayoutExpr
    x2: IntOrFloatOrLayoutExpr
    y1: IntOrFloatOrLayoutExpr
    y2: IntOrFloatOrLayoutExpr
    z_level: int
    stroke: Stroke | None
    fill_color: str | None


class BaseRect:
    def __init__(
        self,
        p1: Sv[Point],
        p2: Sv[Point],
        *,
        stroke: Sn[Stroke] = None,
        fill_color: Sn[str] = None,
        z_level: Sn[int] = None,
        show: BoolStepDef = True,
    ):
        sv_check(p1, check_is_point)
        sv_check(p2, check_is_point)
        sn_check(stroke, check_is_stroke)
        sn_check(fill_color, check_color)
        sn_check(z_level, check_is_int)
        self.show = parse_bool_steps(show)
        self.p1 = p1
        self.p2 = p2
        self.stroke = stroke
        self.fill_color = fill_color
        self.z_level = z_level

    def to_raw(self, step: Step, ctx):
        if not get_step(self.show, step):
            return None
        p1 = get_step(self.p1, step).get_step(step)
        p2 = get_step(self.p2, step).get_step(step)
        stroke = get_step(self.stroke, step)
        if stroke is not None:
            stroke = stroke.to_raw(step)
        return RawRect(
            shape=self.shape,
            x1=p1.x,
            y1=p1.y,
            x2=p2.x,
            y2=p2.y,
            stroke=stroke,
            fill_color=get_step(self.fill_color, step),
            z_level=get_step(self.z_level, step, ctx.z_level),
        )


class Rect(BaseRect):
    shape = 0

class Oval(BaseRect):
    shape = 1



@dataclass
class Arrow:
    """
    Represents an SVG arrow head. Can be attached to the start or end points of lines.
    """

    size: float = 10
    """Size of the arrow head in pixels."""

    angle: float = 40
    """Angle of the arrow head."""

    color: str | None = None
    """Color of arrow, if None color is taken from path"""

    stroke_width: float | None = None
    """If None then a filled arrow is drawn, if float then stroked arrow is drawn with the given stroke width"""

    inner_point: float | None = None
    """ Shape of the arrow head.

        * < 1.0 -> Sharper arrow.
        * = 1.0 -> Normal arrow.
        * > 1.0 -> Diamond shape arrow.
    """


class Path:
    def __init__(
        self,
        *,
        stroke: Stroke | None = None,
        fill_color: str | None = None,
        arrow_start: Arrow | None = None,
        arrow_end: Arrow | None = None,
    ):
        self.stroke = stroke
        self.fill_color = fill_color
        self.commands = []
        self.points = []
        self.arrow_start = arrow_start
        self.arrow_end = arrow_end

    @staticmethod
    def oval(
        x1: PathValue,
        y1: PathValue,
        x2: PathValue,
        y2: PathValue,
        *,
        stroke: Stroke | None = None,
        fill_color: str | None = None,
    ):
        path = Path(stroke=stroke, fill_color=fill_color)
        path.commands.append("oval")
        path.points = [x1, y1, x2, y2]
        return path

    @property
    def last_point(self):
        """
        Returns a last point in the path, if path is empty, returns 0,0
        :return: A tuple (x, y)
        """
        if len(self.points) < 2:
            return 0, 0
        else:
            return self.points[-2], self.points[-1]

    def close(self):
        self.commands.append("close")
        return self

    def move_to(self, x: PathValue, y: PathValue):
        self.commands.append("move")
        self.points.append(x)
        self.points.append(y)
        return self

    def move_by(self, x: PathValue, y: PathValue) -> "Path":
        """
        Perform a move relative to the last point of the path.
        If path is empty, it starts from 0,0
        """
        x_old, y_old = self.last_point

        return self.move_to(x_old + x, y_old + y)

    def line_to(self, x: PathValue, y: PathValue):
        self.commands.append("line")
        self.points.append(x)
        self.points.append(y)
        return self

    def line_by(self, x: PathValue, y: PathValue):
        """
        Draw a line relative to the last point of the path.
        If path is empty, it starts from 0,0
        """
        x_old, y_old = self.last_point
        return self.line_to(x_old + x, y_old + y)

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
