from dataclasses import dataclass

from .resources import Resources
from .basictypes import Position, Size
from . import nelsie as nelsie_rs


@dataclass
class RawBox:
    x: Position
    y: Position
    width: Size
    height: Size
    bg_color: str | None
    children: list["RawBox"]


class RawPage:
    def __init__(self, root: RawBox, width: float, height: float, bg_color: str):
        self.width = width
        self.height = height
        self.bg_color = bg_color
        self.root = root


class Document:
    def __init__(self, resources: Resources, pages: list[RawPage]):
        self.pages = pages
        self.resources = resources

    def render_png_dir(self, path: str):
        nelsie_rs.render_png_dir(self.resources, self.pages, path)

    def render_svg_dir(self, path: str):
        nelsie_rs.render_svg_dir(self.resources, self.pages, path)

    def render_pdf_file(self, path: str):
        nelsie_rs.render_pdf_file(self.resources, self.pages, path)
