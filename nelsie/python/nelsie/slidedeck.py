from copy import copy
from typing import Literal, Iterable, Sequence, Callable

from nelsie.utils import check_is_int_or_float, check_is_type, check_is_str

from .box import BoxBuilderMixin, traverse_children
from .counters import CounterStorage
from .resources import Resources
from .steps import (
    Step,
    Sv,
    Sn,
    step_compare_key,
    is_visible,
    BoolStepDef,
    parse_bool_steps,
)
from . import nelsie as nelsie_rs
from .textstyle import (
    DEFAULT_TEXT_STYLE,
    TextStyle,
    DEFAULT_CODE_STYLE,
    check_is_text_style,
)

type SlideCallback = Callable[["Slide", CounterStorage, CounterStorage], "Slide"]


class Slide(BoxBuilderMixin):
    def __init__(
        self,
        width: Sv[float],
        height: Sv[float],
        bg_color: Sv[str],
        name: str,
        init_steps: Iterable[Step],
        counters: Sequence[str],
        postprocess_fn: SlideCallback | None = None,
        debug_steps: bool = False,
        debug_layout: bool | str = False,
    ):
        self.width = width
        self.height = height
        self.bg_color = bg_color
        self.name = name
        self.children = []
        self.init_steps = init_steps
        self.counters = counters
        self.postprocess_fn = postprocess_fn
        self.subslides = None
        self.debug_steps = debug_steps
        self.debug_layout = debug_layout

        self._text_styles = None
        self._extra_steps = None
        self._ignore_steps = None

    def add(self, box):
        self.children.append(box)

    def slide_at(self, step, **kwargs):
        def helper(fn):
            slide = self.new_slide_at(step, **kwargs)
            fn(slide)
            return slide

        return helper

    def new_slide_at(
        self,
        step: Step,
        *,
        width: Sv[float | None] = None,
        height: Sv[float | None] = None,
        name: str = "",
        bg_color: Sv[str | None] = None,
        init_steps: Iterable[Step] = (1,),
        counters=(),
        postprocess_fn: SlideCallback | None = None,
        debug_steps: bool = False,
        debug_layout: bool | str = False,
    ):
        if width is None:
            width = self.width
        if height is None:
            height = self.height
        if bg_color is None:
            bg_color = self.bg_color
        slide = Slide(
            width,
            height,
            bg_color,
            name,
            init_steps,
            counters,
            postprocess_fn,
            debug_steps,
            debug_layout,
        )
        if self.subslides is None:
            self.subslides = {}
        if step not in self.subslides:
            self.subslides[step] = [slide]
        else:
            self.subslides[step].append(slide)
        return slide

    def insert_step(self, step: Step):
        if self._extra_steps is None:
            self._extra_steps = set()
        self._extra_steps.add(step)

    def ignore_steps(self, ignored_steps: BoolStepDef):
        self._ignore_steps = parse_bool_steps(ignored_steps)

    def _set_style(self, name: str, style: Sn[TextStyle]):
        if self._text_styles is None:
            self._text_styles = {}
        self._text_styles[name] = style

    def _get_style(self, name: str) -> Sn[TextStyle] | None:
        if self._text_styles is not None:
            return self._text_styles.get(name)

    def traverse_tree(self, shared_data, steps: set[Step]):
        traverse_children(self.children, shared_data, steps)

    def copy(self):
        """
        Return copy of slide that is safe to add new boxes into it
        """
        slide = copy(self)
        slide.children = slide.children[:]
        return slide


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
        *,
        width: Sv[float | None] = None,
        height: Sv[float | None] = None,
        name: str = "",
        bg_color: Sv[str | None] = None,
        init_steps: Iterable[Step] = (1,),
        counters=(),
        postprocess_fn: SlideCallback | None = None,
        debug_steps: bool = False,
        debug_layout: bool | str = False,
    ):
        if width is None:
            width = self.width
        if height is None:
            height = self.height
        if bg_color is None:
            bg_color = self.bg_color
        slide = Slide(
            width,
            height,
            bg_color,
            name,
            init_steps,
            counters,
            postprocess_fn,
            debug_steps,
            debug_layout,
        )
        self.slides.append(slide)
        return slide

    def slide(
        self,
        *,
        width: float | None = None,
        height: float | None = None,
        name: str = "",
        bg_color: str | None = None,
        init_steps: Iterable[Step] = (1,),
        ignore: bool = False,
        counters: Sequence[str] = (),
        postprocess_fn: SlideCallback | None = None,
        debug_steps: bool = False,
        debug_layout: bool = False,
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
                counters=counters,
                postprocess_fn=postprocess_fn,
                debug_steps=debug_steps,
                debug_layout=debug_layout,
            )
            fn(slide)
            return slide

        return helper

    def _create_doc(self):
        from .toraw import Document, slide_to_raw
        from .steps_extract import extract_steps

        shared_data = {}
        raw_pages = []
        total_counter = CounterStorage()

        slide_steps = {}

        def gather_steps(slide):
            if slide.subslides is not None:
                for v in slide.subslides.values():
                    for s in v:
                        gather_steps(s)
            steps = set(slide.init_steps)
            slide.traverse_tree(shared_data, steps)
            extract_steps(slide, steps)
            if slide.subslides is not None:
                steps.update(slide.subslides.keys())
            if slide._extra_steps:
                steps.update(slide._extra_steps)
            steps = [s for s in steps if is_visible(s, slide._ignore_steps)]
            steps.sort(key=step_compare_key)
            slide_steps[slide] = steps
            total_counter.increment_page(slide.counters, len(steps))
            total_counter.increment_slide(slide.counters)

        def process_slide(slide):
            steps = slide_steps[slide]
            current_counter.increment_slide(slide.counters)
            for step in steps:
                if slide.subslides is not None:
                    parent_inserted = False
                    subslides = slide.subslides.get(step)
                    if subslides is not None:
                        for s in subslides:
                            if not parent_inserted:
                                parent_inserted = True
                                current_counter.increment_page(slide.counters)
                                page = slide_to_raw(
                                    self.resources,
                                    slide,
                                    step,
                                    self,
                                    shared_data,
                                    current_counter,
                                    total_counter,
                                )
                                raw_pages.append(page)
                            process_slide(s)
                current_counter.increment_page(slide.counters)
                page = slide_to_raw(
                    self.resources,
                    slide,
                    step,
                    self,
                    shared_data,
                    current_counter,
                    total_counter,
                )
                raw_pages.append(page)

        for slide in self.slides:
            gather_steps(slide)

        current_counter = CounterStorage()

        for slide in self.slides:
            process_slide(slide)

        return Document(self.resources, raw_pages)

    def render(
        self,
        path: str | None,
        format: Literal["pdf", "png", "svg"] = "pdf",
        *,
        compression_level: int = 1,
        n_threads: int | None = None,
        progressbar: bool = True,
    ):
        """
        Render slides

        If format is "pdf" then a single PDF file is created. If format is "svg" or "png" then
        `path` specifies a directory where the slides are created as an individual files.

        If `path` is None then objects are not written to the file system, and they are returned as python objects
        from the method call.

        `compression_level` defines the level of compression for PDF, allowed ranges are 0-10
        (0 = no compression, 1 = fast compression, 10 = maximal compression)
        """
        doc = self._create_doc()
        return doc.render(path, format, compression_level, n_threads, progressbar)
