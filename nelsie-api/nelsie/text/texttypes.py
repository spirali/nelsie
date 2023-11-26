from dataclasses import dataclass

from .manager import SteppedTextStyle


@dataclass(frozen=True)
class StyledSpan:
    start: int
    length: int
    style_idx: int


@dataclass(frozen=True)
class StyledLine:
    spans: list[StyledSpan]
    text: str


@dataclass(frozen=True)
class SteppedStyledText:
    styled_lines: list[StyledLine]
    # SteppedTextStyle is intermediate product, for export it has tobe changed to ExportStepValue
    styles: list[SteppedTextStyle]
    default_font_size: float
    default_line_spacing: float
