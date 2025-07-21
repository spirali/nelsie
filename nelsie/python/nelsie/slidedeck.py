from .box import BoxBuilderMixin, Box
from .resources import Resources


class Slide(BoxBuilderMixin):

    def __init__(self, width: float, height: float, bg_color: str, name: str):
        self.width = width
        self.height = height
        self.bg_color = bg_color
        self.name = name
        self.children = []

    def add(self, box: Box):
        self.children.append(box)


class SlideDeck:
    def __init__(
            self,
            *,
            width: float = 1024,
            height: float = 768,
            bg_color: str = "white",
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

        self.width = width
        self.height = height
        self.bg_color = bg_color
        self.image_directory = image_directory
        self.resources = resources
        self.default_code_theme = default_code_theme
        self.default_code_language = default_code_language
        self.slides = []

    def new_slide(self, width: float | None, height: float | None, bg_color: str | None, name: str = ""):
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
