from nelsie.steps.insteps import InSteps, to_steps
from .textstyle import TextStyle, DEFAULT_STYLE

SteppedTextStyle = TextStyle | InSteps[TextStyle]


class TextStyleManager:
    def __init__(self, styles: dict[str, SteppedTextStyle]):
        self._styles: dict[str, SteppedTextStyle] = styles

    def set_style(self, name: str, style: SteppedTextStyle):
        if name == "default":
            style = DEFAULT_STYLE.update(style)
        self._styles[name] = style

    def update_style(self, name: str, style: SteppedTextStyle):
        old_style = self.get_style(name)
        self._styles[name] = update_stepped_text_style(old_style, style)

    def get_style(self, name: str) -> SteppedTextStyle:
        style = self._styles.get(name)
        if style is None:
            raise Exception(f"Style '{name}' does not found")
        return style

    def get_final_style(self, name: str) -> SteppedTextStyle:
        style = self.get_style(name)
        if name == "default":
            return style
        return self._styles["default"].update(style)

    def copy(self) -> "TextStyleManager":
        return TextStyleManager(self._styles.copy())


class TextStylesProviderMixin:
    style_manager: TextStyleManager

    def set_style(self, name: str, style: SteppedTextStyle):
        self.style_manager.set_style(name, style)

    def update_style(self, name: str, style: SteppedTextStyle):
        self.style_manager.update_style(name, style)

    def get_style(self, name: str) -> TextStyle:
        self.style_manager.get_style(name)


def update_stepped_text_style(
    style1: SteppedTextStyle, style2: SteppedTextStyle
) -> SteppedTextStyle:
    if isinstance(style1, TextStyle) and isinstance(style2, TextStyle):
        return style1.update(style2)
    style1 = to_steps(style1)
    style2 = to_steps(style2)
    return style1.zip(style2).map(lambda pair: pair[0].update(pair[1]))
