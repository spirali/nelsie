from dataclasses import dataclass

Size = int | float | str
Position = int | float | str | None


@dataclass
class Stroke:
    color: str
    width: float = 1.0
