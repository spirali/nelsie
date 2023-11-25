from dataclasses import dataclass, field

Size = int | float | str
Position = int | float | str | None


@dataclass
class Stroke:
    color: str
    width: float = 1.0
    dash_array: list[float] = field(default_factory=list)
    dash_offset: float = 0.0
