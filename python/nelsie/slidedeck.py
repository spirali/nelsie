from typing import Optional

from .box import Box, BoxBuilder
from .render import render_slides


class Slide(BoxBuilder):
    def __init__(self, width: float, height: float, bg_color: str):
        self.width = width
        self.height = height
        self.root_box = Box(self, self.width, self.height, bg_color=bg_color)

    def get_slide(self):
        return self

    def add_box(self, box: Box):
        self.root_box.add_box(box)

    def render(self):
        return {
            "width": self.width,
            "height": self.height,
            "node": self.root_box.render(),
        }


class SlideDeck:
    def __init__(
        self,
        *,
        nelsie_bin: str,
        width: float = 1024,
        height: float = 768,
        bg_color: str = "white"
    ):
        self.nelsie_bin = nelsie_bin
        self.width = width
        self.height = height
        self.bg_color = bg_color

        self.slides: list[Slide] = []

    def new_slide(
        self,
        width: Optional[float] = None,
        height: Optional[float] = None,
        bg_color: Optional[str] = None,
    ):
        slide = Slide(
            width=width or self.width,
            height=height or self.height,
            bg_color=bg_color or self.bg_color,
        )
        self.slides.append(slide)
        return slide

    def render(
        self,
        *,
        output_pdf: Optional[str] = None,
        output_svg: Optional[str] = None,
        output_png: Optional[str] = None,
        debug: bool = False
    ):
        if output_pdf is None and output_png is None and output_svg is None:
            raise Exception("No output file is defined")
        root = {"slides": [slide.render() for slide in self.slides]}
        render_slides(self.nelsie_bin, root, output_pdf, output_svg, output_png, debug)
