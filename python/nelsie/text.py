from dataclasses import dataclass

from .utils import unpack_dataclass


@dataclass(frozen=True)
class TextStyle:
    color: str | None = None
    size: float | None = None
    line_spacing: float | None = None

    def update(self, other) -> "TextStyle":
        return TextStyle(
            *[
                b if b is not None else a
                for (a, b) in zip(unpack_dataclass(self), unpack_dataclass(other))
            ]
        )


@dataclass(frozen=True)
class StyledChunk:
    start: int
    end: int
    style: TextStyle


@dataclass(frozen=True)
class StyledText:
    lines: list[list[StyledChunk]]
    text: str


def parse_text(text: str):
    pass
