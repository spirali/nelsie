import pathlib
from typing import Literal

from . import nelsie as nelsie_rs
from .basictypes import parse_debug_layout
from .box import Box, BoxBuilder
from .textstyle import TextStyle, _data_to_text_style

Resources = nelsie_rs.Resources


class Slide(BoxBuilder):
    """
    Represents a slide in a slide deck.

    It should be created via calling method `.new_slide()` on a deck via decorator `@deck.slide()`
    """

    def __init__(
        self,
        deck: "SlideDeck",
        slide_id: int,
        name: str,
        image_directory: str,
        debug_layout: str | None,
    ):
        """
        @private
        """
        self.deck = deck
        self._slide_id = slide_id
        self.name = name
        self.image_directory = image_directory
        self.debug_layout = debug_layout
        self.root_box = Box(deck, self, [], 0, name, 0)

    def get_box(self):
        """
        @private
        """
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
        default_monospace_font: str | None = None,
        default_code_theme: str = "InspiredGitHub",
        default_code_language: str | None = None,
    ):
        """
        A top-level class of Nelsie. It represents a set of slides.

        Arguments:
        * width - default width of a slide (could be overriden for each slide)
        * height - default width of a slide (could be overriden for each slide)
        * bg_color - default background color a slide (could be overriden for each slide)
        * image_directory - default path where images are searched for (could be overriden for each slide)
        * resource - Resource instance, if None a new instance is created
        * default_font - Name of default font
        * default_monospace_font - Name of the default monospace font
        * default_code_theme - Name of default theme for syntax highlighting (.code() method):
            Available themes:
            * "base16-ocean.dark"
            * "base16-eighties.dark"
            * "base16-mocha.dark"
            * "base16-ocean.light"
            * "InspiredGitHub"
            * "Solarized (dark)"
            * "Solarized (light)"
        * default_code_language - Default language to use for syntax highlighting (.code() method)
        """
        if resources is None:
            resources = Resources()

        self.width = width
        self.height = height
        self.bg_color = bg_color
        self.image_directory = image_directory
        self.resources = resources
        self.default_code_theme = default_code_theme
        self.default_code_language = default_code_language
        self._deck = nelsie_rs.Deck(resources, default_font, default_monospace_font)

    def set_style(self, name: str, style: TextStyle):
        self._deck.set_style(self.resources, name, style, False, None, None)

    def update_style(self, name: str, style: TextStyle):
        self._deck.set_style(self.resources, name, style, True, None, None)

    def get_style(self, name: str, step: int = 1) -> TextStyle:
        return _data_to_text_style(self._deck.get_style(name, step, None, None))

    def new_slide(
        self,
        width: float | None = None,
        height: float | None = None,
        bg_color: str | None = None,
        image_directory: str | None = None,
        name: str = "",
        debug_layout: bool | str = False,
        counters: list[str] | None = None,
    ) -> Slide:
        """
        Creates a new slide in the slide deck.
        """
        if width is None:
            width = self.width
        if height is None:
            height = self.height
        if bg_color is None:
            bg_color = self.bg_color
        if image_directory is None:
            image_directory = self.image_directory
        debug_layout = parse_debug_layout(debug_layout)
        slide_id = self._deck.new_slide(width, height, bg_color, name, counters)
        return Slide(self, slide_id, name, image_directory, debug_layout)

    def slide(
        self,
        width: float | None = None,
        height: float | None = None,
        bg_color: str | None = None,
        image_directory: str | None = None,
        name: str = "",
        debug_layout: bool | str = False,
        counters: list[str] | None = None,
    ):
        """
        Decorator for creating new slide.
        It immediately calls the decorated function that should define content of the slide.
        Slide is automatically added into the deck.

        Example:
        ```python
        deck = SlideDeck()

        @deck.slide()
        def my_first_slide(slide):
            slide.text("Hello!")
        ```
        """

        def helper(fn):
            slide = self.new_slide(width, height, bg_color, image_directory, name, debug_layout, counters)
            return fn(slide)

        return helper

    def render(
        self,
        path: str | pathlib.Path | None,
        output_format: Literal["pdf"] | Literal["svg"] | Literal["png"] = "pdf",
        *,
        verbose: int = 1,
    ) -> None | list[bytes]:
        """
        Render slides

        If format is "pdf" then a single PDF file is created. If format is "svg" or "png" then
        `path` specifies a directory where the slides are created as an individual files.

        If `path` is None then objects are not written to the file system, and they are returned as python objects
        from the method call.
        """
        if path:
            path = str(path)
        return self._deck.render(self.resources, verbose, output_format, path)
