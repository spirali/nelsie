from typing import Literal, Iterable

from nelsie.utils import check_is_int_or_float, check_is_type, check_is_str

from .box import BoxBuilderMixin, Box, traverse_children
from .resources import Resources
from .steps import Step, Sv, get_step, Sn, StepVal, sn_apply
from . import nelsie as nelsie_rs
from .textstyle import DEFAULT_TEXT_STYLE, TextStyle, DEFAULT_CODE_STYLE, merge_in_step, check_is_text_style
from .shapes import Path, Rect, Oval


class Slide(BoxBuilderMixin):
    def __init__(self, width: Sv[float], height: Sv[float], bg_color: Sv[str], name: str, init_steps: Iterable[Step]):
        self.width = width
        self.height = height
        self.bg_color = bg_color
        self.name = name
        self.children = []
        self.init_steps = init_steps

        self._text_styles = None
        self._extra_steps = None

    def add(self, box):
        self.children.append(box)

    def insert_step(self, step: Step):
        if self._extra_steps is None:
            self._extra_steps = set()
        self._extra_steps.add(step)

    def _set_style(self, name: str, style: Sn[TextStyle]):
        if self._text_styles is None:
            self._text_styles = {}
        self._text_styles[name] = style

    def _get_style(self, name: str) -> Sn[TextStyle] | None:
        if self._text_styles is not None:
            return self._text_styles.get(name)

    def traverse_tree(self, shared_data, steps: set[Step]):
        traverse_children(self.children, shared_data, steps)


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
        else:
            check_is_type(resources, Resources)

        nelsie_rs.check_color(bg_color)
        check_is_int_or_float(width)
        check_is_int_or_float(height)

        if text_style is not None:
            check_is_text_style(text_style)
            text_style = DEFAULT_TEXT_STYLE.merge(text_style)
        else:
            text_style = DEFAULT_TEXT_STYLE

        if code_style is not None:
            check_is_text_style(code_style)

        self.width = width
        self.height = height
        self.bg_color = bg_color
        self.resources = resources
        self.default_code_theme = default_code_theme
        self.default_code_language = default_code_language
        self._text_styles = {
            "default": text_style,
            "code": code_style,
        }
        self.slides = []

    def get_style(self, name: str) -> TextStyle | None:
        return self._text_styles.get(name)

    def set_style(self, name: str, style: TextStyle):
        if name == "default":
            self.update_style(name, style)
            return
        check_is_str(name)
        check_is_text_style(style)
        self._text_styles[name] = style

    def update_style(self, name: str, style: TextStyle):
        check_is_str(name)
        check_is_text_style(style)
        old_style = self._text_styles.get(name)
        if style is None:
            self._text_styles[name] = style
        else:
            self._text_styles[name] = old_style.merge(style)

    def new_slide(
        self,
        width: Sv[float | None] = None,
        height: Sv[float | None] = None,
        bg_color: Sv[str | None] = None,
        init_steps: Iterable[Step] = (1,),
        name: str = "",
    ):
        if width is None:
            width = self.width
        if height is None:
            height = self.height
        if bg_color is None:
            bg_color = self.bg_color
        slide = Slide(width, height, bg_color, name, init_steps)
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
        init_steps: Iterable[Step] = (1,),
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
                init_steps=init_steps,
            )
            fn(slide)
            return slide

        return helper

    def _create_doc(self):
        from .toraw import Document, slide_to_raw
        from .steps_extract import extract_steps

        shared_data = {}
        raw_pages = []
        for slide in self.slides:
            steps = set(slide.init_steps)
            slide.traverse_tree(shared_data, steps)
            extract_steps(slide, steps)
            if slide._extra_steps:
                steps.update(slide._extra_steps)
            for step in sorted(steps):
                if isinstance(step, int):
                    if step < 1:
                        continue
                elif step[0] < 1:
                    continue
                page = slide_to_raw(slide, step, self, shared_data)
                raw_pages.append(page)
        return Document(self.resources, raw_pages)

    def render(
        self,
        path: str | None,
        format: Literal["pdf", "png", "svg"] = "pdf",
        compression_level: int = 1,
        n_threads: int | None = None,
    ):
        doc = self._create_doc()
        return doc.render(path, format, compression_level, n_threads)
