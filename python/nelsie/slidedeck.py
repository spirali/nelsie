import pathlib
from typing import Optional

from . import nelsie as nelsie_rs
from .basictypes import parse_debug_layout
from .box import Box, BoxBuilder
from .textstyle import TextStyle, _data_to_text_style

Resources = nelsie_rs.Resources


class Slide(BoxBuilder):
    def __init__(
        self,
        deck: "SlideDeck",
        slide_id: int,
        name: str,
        image_directory: str,
        debug_layout: str | None,
    ):
        self.deck = deck
        self._slide_id = slide_id
        self.name = name
        self.image_directory = image_directory
        self.root_box = Box(deck, self, [], 0, name, 0)

    def get_box(self):
        return self.root_box

    def set_n_steps(self, value: int):
        assert value >= 1
        self.deck._deck.set_n_steps(self._slide_id, value)

    def get_n_steps(self) -> int:
        return self.deck._deck.get_n_steps(self._slide_id)


class SlideDeck:
    def __init__(
        self,
        *,
        width: float = 1024,
        height: float = 768,
        bg_color: str = "white",
        image_directory: str | None = None,
        resources: Resources | None = None,
        default_font: str | None = None,
        default_theme: str = "InspiredGitHub",
    ):
        """
        Available syntax highlight themes:
          * "base16-ocean.dark",
          * "base16-eighties.dark"
          * "base16-mocha.dark"
          * "base16-ocean.light"
          * "InspiredGitHub"
          * "Solarized (dark)"
          * "Solarized (light)"
        """
        if resources is None:
            resources = Resources()

        self.width = width
        self.height = height
        self.bg_color = bg_color
        self.image_directory = image_directory
        self.resources = resources
        self.default_theme = default_theme
        self._deck = nelsie_rs.Deck(resources, default_font)

    def set_style(self, name: str, style: TextStyle):
        self._deck.set_style(self.resources, name, style, False, None, None)

    def update_style(self, name: str, style: TextStyle):
        self._deck.set_style(self.resources, name, style, True, None, None)

    def get_style(self, name: str, step: int = 1) -> TextStyle:
        return _data_to_text_style(self._deck.get_style(name, step, None, None))

    def new_slide(
        self,
        width: Optional[float] = None,
        height: Optional[float] = None,
        bg_color: Optional[str] = None,
        image_directory: str | None = None,
        name: str = "",
        debug_layout: bool | str = False,
    ):
        if width is None:
            width = self.width
        if height is None:
            height = self.height
        if bg_color is None:
            bg_color = self.bg_color
        if image_directory is None:
            image_directory = self.image_directory
        debug_layout = parse_debug_layout(debug_layout)
        slide_id = self._deck.new_slide(width, height, bg_color, name)
        return Slide(self, slide_id, name, image_directory, debug_layout)

    def slide(
        self,
        width: Optional[float] = None,
        height: Optional[float] = None,
        bg_color: Optional[str] = None,
        image_directory: str | None = None,
        name: str = "",
        debug_layout: bool | str = False,
    ):
        def helper(fn):
            slide = self.new_slide(
                width, height, bg_color, image_directory, name, debug_layout
            )
            return fn(slide)

        return helper

    def render(
        self,
        *,
        output_pdf: Optional[str | pathlib.Path] = None,
        output_svg: Optional[str | pathlib.Path] = None,
        output_png: Optional[str | pathlib.Path] = None,
        debug: bool = False,
    ):
        if output_pdf:
            output_pdf = str(output_pdf)
        if output_png:
            output_png = str(output_png)
        if output_svg:
            output_svg = str(output_svg)
        self._deck.render(self.resources, output_pdf, output_svg, output_png)
