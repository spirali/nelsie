from dataclasses import dataclass

from nelsie.nelsie import check_color

from .basictypes import check_position
from .layoutexpr import LayoutExpr, IntOrFloatOrLayoutExpr
from .steps import (
    Sv,
    Sn,
    get_step,
    Step,
    sv_check,
    sn_check,
    BoolStepDef,
    parse_bool_steps,
)
from .utils import check_is_int_or_float, check_is_type, check_is_int

PathValue = int | float | LayoutExpr


@dataclass(frozen=True)
class Point:
    x: Sv[IntOrFloatOrLayoutExpr]
    y: Sv[IntOrFloatOrLayoutExpr]

    def __post_init__(self):
        sv_check(self.x, check_position)
        sv_check(self.y, check_position)

    def move_by(self, x: IntOrFloatOrLayoutExpr, y: IntOrFloatOrLayoutExpr) -> "Point":
        return Point(self.x + x, self.y + y)


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
    y1: IntOrFloatOrLayoutExpr
    x2: IntOrFloatOrLayoutExpr
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
        stroke = get_step(self.stroke, step)
        if stroke is not None:
            stroke = stroke.to_raw(step)
        return RawRect(
            shape=self.shape,
            x1=get_step(self.p1.x, step),
            y1=get_step(self.p1.y, step),
            x2=get_step(self.p2.x, step),
            y2=get_step(self.p2.y, step),
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

    size: Sv[float] = 10
    """Size of the arrow head in pixels."""

    angle: Sv[float] = 40
    """Angle of the arrow head."""

    color: Sn[str] = None
    """Color of arrow, if None color is taken from path"""

    stroke_width: Sn[float] = None
    """If None then a filled arrow is drawn, if float then stroked arrow is drawn with the given stroke width"""

    inner_point: Sn[float] = None
    """ Shape of the arrow head.

        * < 1.0 -> Sharper arrow.
        * = 1.0 -> Normal arrow.
        * > 1.0 -> Diamond shape arrow.
    """

    def __post_init__(self):
        sv_check(self.size, check_is_int_or_float)
        sv_check(self.angle, check_is_int_or_float)
        sn_check(self.color, check_color)
        sn_check(self.stroke_width, check_is_int_or_float)
        sn_check(self.inner_point, check_is_int_or_float)

    def at_step(self, step: Step):
        return Arrow(
            get_step(self.size, step),
            get_step(self.angle, step),
            get_step(self.color, step),
            get_step(self.stroke_width, step),
            get_step(self.inner_point, step),
        )


def check_is_arrow(obj):
    check_is_type(obj, Arrow)


@dataclass
class RawPath:
    stroke: Stroke | None
    fill_color: str | None
    arrow_start: Arrow | None
    arrow_end: Arrow | None
    commands: list[str]
    points: list[LayoutExpr]
    z_level: int


class Path:
    def __init__(
        self,
        *,
        stroke: Sn[Stroke] = None,
        fill_color: Sn[str] = None,
        arrow_start: Sn[Arrow] = None,
        arrow_end: Sn[Arrow] = None,
        z_level: Sn[int] = None,
        show: BoolStepDef = True,
    ):
        sn_check(stroke, check_is_stroke)
        sn_check(fill_color, check_color)
        sn_check(arrow_start, check_is_arrow)
        sn_check(arrow_end, check_is_arrow)
        sn_check(z_level, check_is_int)
        self.show = parse_bool_steps(show)
        self.stroke = stroke
        self.fill_color = fill_color
        self.commands = []
        self.points = []
        self.arrow_start = arrow_start
        self.arrow_end = arrow_end
        self.z_level = z_level

    @property
    def last_point(self):
        """
        Returns a last point in the path, if path is empty, returns 0,0
        :return: Point
        """
        if len(self.points) < 1:
            return Point(0, 0)
        else:
            return self.points[-1]

    def close(self):
        self.commands.append("close")
        return self

    def move_to(self, point: Point):
        check_is_point(point)
        self.commands.append("move")
        self.points.append(point)
        return self

    def move_by(self, x: PathValue, y: PathValue) -> "Path":
        """
        Perform a move relative to the last point of the path.
        If path is empty, it starts from 0,0
        """
        point = self.last_point

        return self.move_to(Point(point.x + x, point.y + y))

    def line_to(self, point: Point):
        check_is_point(point)
        self.commands.append("line")
        self.points.append(point)
        return self

    def line_by(self, x: PathValue, y: PathValue):
        """
        Draw a line relative to the last point of the path.
        If path is empty, it starts from 0,0
        """
        point = self.last_point
        return self.line_to(Point(point.x + x, point.y + y))

    def quad_to(self, point1: Point, point: Point):
        check_is_point(point1)
        check_is_point(point)
        self.commands.append("quad")
        self.points.append(point1)
        self.points.append(point)
        return self

    def cubic_to(
        self,
        point1: Point,
        point2: Point,
        point: Point,
    ):
        check_is_point(point1)
        check_is_point(point2)
        check_is_point(point)

        self.commands.append("cubic")
        self.points.append(point1)
        self.points.append(point2)
        self.points.append(point)
        return self

    def to_raw(self, step, ctx):
        if not get_step(self.show, step, False):
            return None
        stroke = get_step(self.stroke, step)
        if stroke is not None:
            stroke = stroke.to_raw(step)
        fill_color = get_step(self.fill_color, step)

        arrow_start = get_step(self.arrow_start, step)
        if arrow_start is not None:
            arrow_start = arrow_start.at_step(step)

        arrow_end = get_step(self.arrow_end, step)
        if arrow_end is not None:
            arrow_end = arrow_end.at_step(step)

        points = []
        for p in self.points:
            points.append(get_step(p.x, step))
            points.append(get_step(p.y, step))

        return RawPath(
            stroke=stroke,
            fill_color=fill_color,
            arrow_start=arrow_start,
            arrow_end=arrow_end,
            commands=self.commands,
            points=points,
            z_level=get_step(self.z_level, step, ctx.z_level),
        )
