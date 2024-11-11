import pathlib
import os
import importlib.resources as import_resources
from typing import Literal, List

from . import nelsie as nelsie_rs
from .basictypes import parse_debug_layout
from .box import Box, BoxBuilder
from .insteps import Step
from .textstyle import TextStyle, _data_to_text_style

from . import data

BUILTIN_FONTS_DIR = os.path.abspath(import_resources.files(data) / "fonts")


class Resources:
    def __init__(
        self,
        *,
        builtin_fonts: bool = True,
        system_fonts: bool = False,
        default_code_syntaxes: bool = True,
        default_code_themes: bool = True,
    ):
        self._resources = nelsie_rs.Resources(system_fonts, default_code_syntaxes, default_code_themes)
        if builtin_fonts:
            self._resources.load_fonts_dir(BUILTIN_FONTS_DIR)
            self._resources.set_generic_family("sans-serif", "DejaVu Sans")
            self._resources.set_generic_family("monospace", "DejaVu Sans Mono")

    def set_sans_serif(self, font_name):
        self._resources.set_generic_family("sans-serif", font_name)

    def set_monospace(self, font_name):
        self._resources.set_generic_family("monospace", font_name)

    def set_serif(self, font_name):
        self._resources.set_generic_family("serif", font_name)

    def load_code_syntax_dir(self, path: str):
        self._resources.load_code_syntax_dir(path)

    def load_code_theme_dir(self, path: str):
        self._resources.load_code_theme_dir(path)

    def load_fonts_dir(self, path: str):
        self._resources.load_fonts_dir(path)

    def syntaxes(self) -> list[tuple[str, list[str]]]:
        return self._resources.syntaxes()

    def themes(self) -> list[str]:
        return self._resources.themes()


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

    def insert_step(self, value: Step):
        self.deck._deck.insert_step(self._slide_id, value)

    def remove_step(self, value: Step):
        self.deck._deck.remove_step(self._slide_id, value)

    def remove_steps_below(self, value: Step):
        self.deck._deck.remove_steps_below(self._slide_id, value)

    def remove_steps_above(self, value: Step):
        self.deck._deck.remove_steps_above(self._slide_id, value)

    def set_steps(self, values: set[Step] | frozenset[Step]):
        self.deck._deck.set_steps(self._slide_id, values)

    def get_steps(self) -> list[Step]:
        return self.deck._deck.get_steps(self._slide_id)

    def new_slide_at(self, step: int, **slide_kwargs):
        slide_kwargs["parent_slide"] = (self._slide_id, step)
        return self.deck.new_slide(**slide_kwargs)

    def slide_at(self, step: int, **slide_kwargs):
        slide_kwargs["parent_slide"] = (self._slide_id, step)
        return self.deck.slide(**slide_kwargs)


class SlideDeck:
    def __init__(
        self,
        *,
        width: float = 1024,
        height: float = 768,
        bg_color: str = "white",
        image_directory: str | None = None,
        resources: Resources | None = None,
        default_font: str = "sans-serif",
        default_monospace_font: str = "monospace",
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
        self._deck = nelsie_rs.Deck(default_font, default_monospace_font)
        self._slides: List[Slide] = []

    def set_style(self, name: str, style: TextStyle):
        self._deck.set_style(self.resources._resources, name, style, False, None, None)

    def update_style(self, name: str, style: TextStyle):
        self._deck.set_style(self.resources._resources, name, style, True, None, None)

    def get_style(self, name: str, step: int = 1) -> TextStyle:
        return _data_to_text_style(self._deck.get_style(name, step, None, None))

    def new_slide(
        self,
        *,
        width: float | None = None,
        height: float | None = None,
        bg_color: str | None = None,
        image_directory: str | None = None,
        name: str = "",
        debug_steps: bool = False,
        debug_layout: bool | str = False,
        counters: list[str] | None = None,
        parent_slide: tuple[Slide, int] | None = None,
        step_1: bool = True,
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
        slide_id = self._deck.new_slide(width, height, bg_color, name, step_1, debug_steps, counters, parent_slide)

        slide = Slide(self, slide_id, name, image_directory, debug_layout)
        self._slides.append(slide)
        return slide

    def slide(
        self,
        *,
        width: float | None = None,
        height: float | None = None,
        bg_color: str | None = None,
        image_directory: str | None = None,
        name: str = "",
        debug_steps: bool = False,
        debug_layout: bool | str = False,
        counters: list[str] | None = None,
        parent_slide: tuple[Slide, int] | None = None,
        step_1: bool = True,
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
            slide = self.new_slide(
                width=width,
                height=height,
                bg_color=bg_color,
                image_directory=image_directory,
                name=name,
                debug_steps=debug_steps,
                debug_layout=debug_layout,
                counters=counters,
                parent_slide=parent_slide,
                step_1=step_1,
            )
            fn(slide)
            return slide

        return helper

    def render(
        self,
        path: str | pathlib.Path | None,
        output_format: Literal["pdf"] | Literal["svg"] | Literal["png"] = "pdf",
        *,
        verbose: int = 1,
        compression_level: int = 1,
        n_threads: int | None = None,
    ) -> None | list[bytes]:
        """
        Render slides

        If format is "pdf" then a single PDF file is created. If format is "svg" or "png" then
        `path` specifies a directory where the slides are created as an individual files.

        If `path` is None then objects are not written to the file system, and they are returned as python objects
        from the method call.

        `compression_level` defines the level of compression for PDF, allowed ranges are 0-10
        (0 = no compression, 1 = fast compression, 10 = maximal compression)
        """
        assert 0 <= compression_level <= 10
        if path:
            path = str(path)
        return self._deck.render(
            self.resources._resources,
            verbose,
            output_format,
            compression_level,
            path,
            n_threads,
        )
