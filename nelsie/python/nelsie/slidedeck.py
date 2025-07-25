from typing import Literal

from .box import BoxBuilderMixin, Box
from .doc import RawPage, RawBox, Document
from .resources import Resources
from .steps import Step, Sv, get_step
from . import nelsie as nelsie_rs
from .textstyle import DEFAULT_TEXT_STYLE, TextStyle, DEFAULT_CODE_STYLE, merge_in_step
from .toraw import ToRawContext, box_to_raw


class Slide(BoxBuilderMixin):
    def __init__(self, width: Sv[float], height: Sv[float], bg_color: Sv[str], name: str):
        self.width = width
        self.height = height
        self.bg_color = bg_color
        self.name = name
        self.children = []

        self.text_style = None
        self.code_style = None

    def add(self, box: Box):
        self.children.append(box)

    def to_raw_page(self, step: Step, deck: "SlideDeck") -> RawPage:
        width = get_step(self.width, step, deck.width)
        height = get_step(self.height, step, deck.height)
        text_style = merge_in_step(deck.text_style, self.text_style, step)
        code_style = merge_in_step(deck.code_style, self.code_style, step)
        ctx = ToRawContext(text_style, code_style)
        root = RawBox(
            x=0,
            y=0,
            width=width,
            height=height,
            bg_color=None,
            children=[box_to_raw(child, step, ctx) for child in self.children],
            content=None,
        )
        return RawPage(
            width=width,
            height=height,
            bg_color=get_step(self.bg_color, step, deck.bg_color),
            root=root,
        )


class SlideDeck:
    def __init__(
            self,
            *,
            width: float = 1024,
            height: float = 768,
            bg_color: str = "white",
            text_style: TextStyle | None = None,
            code_style: TextStyle = DEFAULT_CODE_STYLE,
            resources: Resources | None = None,
            default_code_theme: str = "InspiredGitHub",
            default_code_language: str | None = None,
    ):
        """
        A top-level class of Nelsie. It represents a set of slides.

        Arguments:
        * width - default width of a slide (could be overridden for each slide)
        * height - default width of a slide (could be overridden for each slide)
        * bg_color - default background color a slide (could be overridden for each slide)
        * resource - Resource instance, if None a new instance is created
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

        if text_style is not None:
            text_style = DEFAULT_TEXT_STYLE.merge(text_style)
        else:
            text_style = DEFAULT_TEXT_STYLE

        self.width = width
        self.height = height
        self.bg_color = bg_color
        self.resources = resources
        self.default_code_theme = default_code_theme
        self.default_code_language = default_code_language
        self.text_style = text_style
        self.code_style = code_style
        self.slides = []

    def new_slide(
            self,
            width: Sv[float | None] = None,
            height: Sv[float | None] = None,
            bg_color: Sv[str | None] = None,
            name: str = "",
    ):
        if width is None:
            width = self.width
        if height is None:
            height = self.height
        if bg_color is None:
            bg_color = self.bg_color
        slide = Slide(width, height, bg_color, name)
        self.slides.append(slide)
        return slide

    def slide(
            self,
            *,
            width: float | None = None,
            height: float | None = None,
            bg_color: str | None = None,
            name: str = "",
            # debug_steps: bool = False,
            # debug_layout: bool | str = False,
            # counters: list[str] | None = None,
            # parent_slide: tuple[Slide, int] | None = None,
            # step_1: bool = True,
            ignore: bool = False,
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
            if ignore:
                return None
            slide = self.new_slide(
                width=width,
                height=height,
                bg_color=bg_color,
                name=name,
            )
            fn(slide)
            return slide

        return helper

    def _create_doc(self):
        raw_pages = []
        for slide in self.slides:
            # TODO: gather steps
            steps = [1]
            for step in steps:
                raw_pages.append(slide.to_raw_page(step, self))
        return Document(self.resources, raw_pages)

    def render(self, path: str | None, format: Literal["pdf", "png", "svg"] = "pdf", compression_level: int = 1,
               n_threads: int | None = None):
        doc = self._create_doc()
        return doc.render(path, format, compression_level, n_threads)
