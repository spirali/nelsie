from dataclasses import dataclass
from typing import TypeVar, Generic, Literal

T = TypeVar("T")


@dataclass
class PointsSize(Generic[T]):
    points: float


@dataclass
class PercentSize(Generic[T]):
    percent: float


ExportSize = PercentSize | PointsSize | Literal["auto"]


@dataclass
class ExportConstStepValue(Generic[T]):
    const: T


@dataclass
class ExportComplexStepValue(Generic[T]):
    steps: list[T]


ExportStepValue = ExportConstStepValue[T] | ExportComplexStepValue[T]


@dataclass
class ExportNode:
    width: ExportStepValue[ExportSize]
    height: ExportStepValue[ExportSize]

    show: ExportStepValue[bool]
    row: ExportStepValue[bool]
    reverse: ExportStepValue[bool]

    bg_color: ExportStepValue[str]

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
