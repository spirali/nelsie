from dataclasses import dataclass

from .resources import Resources
from .basictypes import Position, Size


@dataclass
class RawBox:
    x: Position
    y: Position
    width: Size
    height: Size
    bg_color: str


class RawPage:

    def __init__(self, root: RawBox, width: float, height: float, bg_color: str = "white"):
        self.width = width
        self.height = height
        self.bg_color = bg_color
        self.root = root


class Document:

    def __init__(self, resources: Resources, pages: list[RawPage]):
        self.pages = pages
        self.resources = resources

    def render_png_dir(self, path: str):
        pass
