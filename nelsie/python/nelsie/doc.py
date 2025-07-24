from dataclasses import dataclass
from typing import Literal

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

    def render(self, path: str | None, format: Literal["pdf", "png", "svg"] = "pdf", compression_level: int = 1,
               n_threads: int | None = None):
        nelsie_rs.render(self.resources._resources, self.pages, path, format, compression_level, n_threads)
