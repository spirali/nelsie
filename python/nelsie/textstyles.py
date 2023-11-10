from dataclasses import dataclass

from .utils import unpack_dataclass
from .colors import check_color


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

    def update(self, other) -> "TextStyle":
        return TextStyle(
            *[
                b if b is not None else a
                for (a, b) in zip(unpack_dataclass(self), unpack_dataclass(other))
            ]
        )


# DEFAULT_STYLE *has to* have all attribute non-None
DEFAULT_STYLE = TextStyle(color="black", size=32, line_spacing=1.2)


class TextStyleManager:
    def __init__(self, styles: dict[str, TextStyle]):
        self._styles: dict[str, TextStyle] = styles

    def set_style(self, name: str, style: TextStyle):
        if name == "default":
            style = DEFAULT_STYLE.update(style)
        self._styles[name] = style

    def update_style(self, name: str, style: TextStyle):
        old_style = self.get_style(name)
        self._styles[name] = old_style.update(style)

    def get_style(self, name: str) -> TextStyle:
        style = self._styles.get(name)
        if style is None:
            raise Exception(f"Style '{name}' does not found")
        return style

    def get_final_style(self, name: str) -> TextStyle:
        style = self.get_style(name)
        if name == "default":
            return style
        return self._styles["default"].update(style)

    def copy(self) -> "TextStyleManager":
        return TextStyleManager(self._styles.copy())


class TextStylesProviderMixin:
    style_manager: TextStyleManager

    def set_style(self, name: str, style: TextStyle):
        self.style_manager.set_style(name, style)

    def update_style(self, name: str, style: TextStyle):
        self.style_manager.update_style(name, style)

    def get_style(self, name: str) -> TextStyle:
        self.style_manager.get_style(name)
