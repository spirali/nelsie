import typing
from typing import Optional
from . import nelsie as nelsie_rs
from .basictypes import parse_debug_layout
from .box import Box, BoxBuilder
from .insteps import InSteps


class Slide(BoxBuilder):

    def __init__(self, deck, slide_id: int, name: str, image_directory: str, debug_layout:  str | None):
        self._deck = deck
        self._slide_id = slide_id
        self.name = name
        self.image_directory = image_directory
        self.root_box = Box(deck, self, [], 0, name, 0)

    def get_box(self):
        return self.root_box


class SlideDeck:
    def __init__(
        self,
        *,
        width: float = 1024,
        height: float = 768,
        bg_color: str = "white",
        image_directory: str | None = None
    ):
        self.width = width
        self.height = height
        self.bg_color = bg_color
        self.image_directory = image_directory
        self._deck = nelsie_rs.Deck()

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
        return Slide(self._deck, slide_id, name, image_directory, debug_layout)

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
        output_pdf: Optional[str] = None,
        output_svg: Optional[str] = None,
        output_png: Optional[str] = None,
        debug: bool = False
    ):
        self._deck.render(output_pdf, output_svg, output_png)