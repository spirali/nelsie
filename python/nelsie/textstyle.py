from dataclasses import dataclass

from nelsie.utils import unpack_dataclass


@dataclass(frozen=True)
class TextStyle:
    font_family: str | list[str] | None = None
    color: str | None = None
    size: float | None = None
    line_spacing: float | None = None

    def __post_init__(self):
        if self.size is not None:
            assert self.size >= 0
        if self.line_spacing is not None:
            assert self.line_spacing >= 0

    def update(self, other: "TextStyle") -> "TextStyle":
        assert isinstance(other, TextStyle)
        return TextStyle(
            *[
                b if b is not None else a
                for (a, b) in zip(unpack_dataclass(self), unpack_dataclass(other))
            ]
        )
