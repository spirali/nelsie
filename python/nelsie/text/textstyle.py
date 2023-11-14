from dataclasses import dataclass

from nelsie.utils import unpack_dataclass
from nelsie.colors import check_color


@dataclass(frozen=True)
class TextStyle:
    color: str | None = None
    size: float | None = None
    line_spacing: float | None = None

    def __post_init__(self):
        if self.color is not None:
            check_color(self.color)
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


# DEFAULT_STYLE *has to* have all attribute non-None
DEFAULT_STYLE = TextStyle(color="black", size=32, line_spacing=1.2)
