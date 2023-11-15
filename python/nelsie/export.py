from dataclasses import dataclass
from typing import TypeVar, Generic, Literal

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


@dataclass
class AutoSize:
    _tag = "auto"


AUTO_SIZE = AutoSize()

ExportSize = FractionSize | PointsSize | AutoSize


@dataclass
class ExportConstStepValue(Generic[T]):
    const: T


@dataclass
class ExportComplexStepValue(Generic[T]):
    steps: list[T]


ExportStepValue = ExportConstStepValue[T] | ExportComplexStepValue[T]


@dataclass(frozen=True)
class ExportStyledText:
    styled_lines: list[StyledLine]
    # SteppedTextStyle is intermediate product, for export it has tobe changed to ExportStepValue
    styles: list[TextStyle]
    default_font_size: float
    default_line_spacing: float


@dataclass
class ExportNode:
    node_id: int

    x: ExportStepValue[None]
    y: ExportStepValue[None]

    width: ExportStepValue[ExportSize]
    height: ExportStepValue[ExportSize]

    show: ExportStepValue[bool]
    row: ExportStepValue[bool]
    reverse: ExportStepValue[bool]

    bg_color: ExportStepValue[str]
    text: ExportStepValue[ExportStyledText]

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
