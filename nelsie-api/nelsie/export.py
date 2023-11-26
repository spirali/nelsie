from dataclasses import dataclass
from typing import Generic, TypeVar

from .basictypes import Stroke
from .layoutexpr import LayoutExpr
from .steps.insteps import InSteps
from .text.textstyle import TextStyle
from .text.texttypes import StyledLine

T = TypeVar("T")


@dataclass
class PointsSize:
    _tag = "points"
    value: float


@dataclass
class FractionSize:
    _tag = "fraction"
    value: float


ExportSize = FractionSize | PointsSize | None


@dataclass
class ExportConstStepValue(Generic[T]):
    const: T


@dataclass
class ExportComplexStepValue(Generic[T]):
    steps: dict[int, T]


ExportStepValue = ExportConstStepValue[T] | ExportComplexStepValue[T]


@dataclass
class ExportPathMove:
    _tag = "move"
    x: LayoutExpr
    y: LayoutExpr


@dataclass
class ExportPathLine:
    _tag = "line"
    x: LayoutExpr
    y: LayoutExpr


@dataclass
class ExportPathQuad:
    _tag = "quad"
    x1: LayoutExpr
    y1: LayoutExpr
    x: LayoutExpr
    y: LayoutExpr


@dataclass
class ExportPathCubic:
    _tag = "quad"
    x1: LayoutExpr
    y1: LayoutExpr
    x2: LayoutExpr
    y2: LayoutExpr
    x: LayoutExpr
    y: LayoutExpr


@dataclass
class ExportPath:
    stroke: Stroke | None
    parts: list[ExportPathMove | ExportPathLine | ExportPathQuad | ExportPathCubic]


@dataclass
class ExportDrawing:
    _tag = "draw"
    paths: InSteps[list[ExportPath]]


@dataclass
class StyledText:
    _tag = "text"
    styled_lines: list[StyledLine]
    styles: list[TextStyle]
    default_font_size: float
    default_line_spacing: float


@dataclass
class Image:
    _tag = "image"
    filename: str
    enable_steps: bool
    shift_steps: int


NodeContent = StyledText | Image | None


@dataclass
class ExportNode:
    _tag = "node"
    node_id: int
    name: str
    debug_layout: str | None

    x: ExportStepValue[None | LayoutExpr]
    y: ExportStepValue[None | LayoutExpr]

    z_level: ExportStepValue[int]

    width: ExportStepValue[ExportSize]
    height: ExportStepValue[ExportSize]

    show: ExportStepValue[bool]
    row: ExportStepValue[bool]
    reverse: ExportStepValue[bool]

    bg_color: ExportStepValue[str]
    content: ExportStepValue[NodeContent]

    children: list["ExportNode"] | None = None


@dataclass
class ExportSlide:
    width: float
    height: float
    n_steps: int
    node: ExportNode


@dataclass
class ExportSlideDeck:
    slides: list[ExportSlide]
