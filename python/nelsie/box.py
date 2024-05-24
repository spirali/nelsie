import os
from dataclasses import dataclass

from .basictypes import (
    AlignItems,
    AlignContent,
    FlexWrap,
    Length,
    LengthAuto,
    Position,
    Size,
    TextAlign,
    parse_debug_layout,
)
from .insteps import InSteps, zip_in_steps
from .layoutexpr import LayoutExpr
from .shapes import Path
from .textstyle import TextStyle, _data_to_text_style
from .watch import watch_path


@dataclass
class TextContent:
    text: str
    style1: TextStyle | str | None
    style2: TextStyle | str | None
    formatting_delimiters: str
    text_align: int
    syntax_language: str | None
    syntax_theme: str | None
    parse_counters: bool


@dataclass
class ImageContent:
    path: str
    enable_steps: bool
    shift_steps: int


NodeContent = ImageContent | TextContent | None
AlignItemsSteps = AlignItems | None | InSteps[AlignItems | None]
AlignContentSteps = AlignContent | None | InSteps[AlignContent | None]


def _resolve_step_keywords(value, parent_box):
    if value == "last":
        return parent_box.slide.get_steps()[-1]
    if value == "last+":
        return f"{'.'.join(str(s) for s in parent_box.slide.get_steps()[-1])}+"
    if value == "next":
        return parent_box.slide.get_steps()[-1][0] + 1
    if value == "next+":
        return f"{parent_box.slide.get_steps()[-1][0] + 1}+"
    return value


class BoxBuilder:
    def get_box(self):
        """
        @private
        """
        raise NotImplementedError

    def set_style(self, name: str, style: TextStyle | InSteps[TextStyle]):
        """
        Set text style under given name.
        """
        box = self.get_box()
        deck = box.deck
        deck._deck.set_style(deck.resources, name, style, False, box.slide._slide_id, box._box_id)

    def update_style(self, name: str, style: TextStyle | InSteps[TextStyle]):
        """
        Load a text style and merge it with the `style` and save it back.
        Throws an exception if a style under given name does not exist.
        """
        box = self.get_box()
        deck = box.deck
        deck._deck.set_style(deck.resources, name, style, True, box.slide._slide_id, box._box_id)

    def get_style(self, name: str, step: int = 1) -> TextStyle:
        """
        Get text style under given name. Style is returned for a given `step`.
        Throws an exception if a style under given name does not exist.

        This function returns text style even when InSteps was used to set the style,
        as it returns text style for a given step.
        """

        box = self.get_box()
        return _data_to_text_style(box.deck._deck.get_style(name, step, box.slide._slide_id, box._box_id))

    def image(self, path: str, enable_steps=True, shift_steps=0, **box_args):
        """
        Create a box with an image. Supported formats: SVG, PNG, JPEG, GIF, ORA
        """
        assert shift_steps >= 0

        slide = self.get_box().slide
        if slide.image_directory is not None:
            path = os.path.join(slide.image_directory, path)
        path = os.path.abspath(path)
        watch_path(path)
        image = ImageContent(
            path=path,
            enable_steps=enable_steps,
            shift_steps=shift_steps,
        )
        box_args.setdefault("name", f"image: {path}")
        return self.box(_content=image, **box_args)

    def text(
        self,
        text: str | InSteps[str],
        style: str | TextStyle | InSteps[TextStyle] | None = None,
        *,
        parse_styles: bool = True,
        style_delimiters: str | None = "~{}",
        tab_width: int = 4,
        align: TextAlign = "start",
        strip=True,
        parse_counters: bool = False,
        **box_args,
    ):
        """
        Create a box with a text.
        """
        box_args.setdefault("name", "text")
        return self._text_box(
            text,
            style,
            None,
            style_delimiters if parse_styles else None,
            tab_width,
            box_args,
            align,
            None,
            None,
            strip,
            parse_counters,
        )

    def code(
        self,
        text: str,
        language: str | None = "default",
        style: str | TextStyle | InSteps[TextStyle] | None = None,
        *,
        theme: str | None = None,
        parse_styles: bool = False,
        style_delimiters: str | None = "~{}",
        tab_width: int = 4,
        align: TextAlign = "start",
        strip: bool = True,
        parse_counters: bool = False,
        **box_args,
    ):
        """
        Create a box with a syntax highlighted text.
        """
        box_args.setdefault("name", "code")
        if theme is None:
            theme = self.get_box().deck.default_code_theme
        if language == "default":
            language = self.get_box().deck.default_code_language
        return self._text_box(
            text,
            "code",
            style,
            style_delimiters if parse_styles else None,
            tab_width,
            box_args,
            align,
            language,
            theme,
            strip,
            parse_counters,
        )

    def _text_box(
        self,
        text,
        style1,
        style2,
        delimiters,
        tab_width,
        box_args,
        align,
        language,
        theme,
        strip,
        parse_counters,
    ):
        def text_preprocess(x):
            if strip:
                x = x.strip()
            return x.replace("\t", " " * tab_width)

        if isinstance(text, str):
            text = text_preprocess(text)
        elif isinstance(text, list):
            text = zip_in_steps(text, "").map(lambda x: text_preprocess("".join(x)))
        elif isinstance(text, InSteps):
            text = text.map(text_preprocess)
        else:
            raise Exception("Invalid type for text")

        if align == "start":
            align = 0
        elif align == "center":
            align = 1
        elif align == "end":
            align = 2
        else:
            raise Exception(f"Invalid alignment: {align}")
        text_content = TextContent(
            text=text,
            style1=style1,
            style2=style2,
            formatting_delimiters=delimiters,
            text_align=align,
            syntax_language=language,
            syntax_theme=theme,
            parse_counters=parse_counters,
        )
        return self.box(_content=text_content, **box_args)

    def draw(self, paths: Path | list[Path] | InSteps[Path | list[Path]]):
        """
        Draw one or paths in the slide.
        """

        if isinstance(paths, Path):
            paths = [paths]
        elif isinstance(paths, InSteps):
            paths = paths.map(lambda p: [p] if isinstance(p, Path) else p)
        box = self.get_box()
        box.deck._deck.draw(box.slide._slide_id, box._box_id, paths)

    def box(
        self,
        *,
        active: bool | str | int | InSteps[bool] = True,
        show: bool | str | int | InSteps[bool] = True,
        z_level: int | InSteps[int] | None = None,
        x: Position | InSteps[Position] = None,
        y: Position | InSteps[Position] = None,
        width: Size | InSteps[Size] = None,
        height: Size | InSteps[Size] = None,
        border_radius: float | InSteps[float] = 0,
        p_left: Length | InSteps[Length] | None = None,
        p_right: Length | InSteps[Length] | None = None,
        p_top: Length | InSteps[Length] | None = None,
        p_bottom: Length | InSteps[Length] | None = None,
        p_x: Length | InSteps[Length] = 0,
        p_y: Length | InSteps[Length] = 0,
        m_left: LengthAuto | InSteps[LengthAuto] | None = None,
        m_right: LengthAuto | InSteps[LengthAuto] | None = None,
        m_top: LengthAuto | InSteps[LengthAuto] | None = None,
        m_bottom: LengthAuto | InSteps[LengthAuto] | None = None,
        m_x: LengthAuto | InSteps[LengthAuto] = 0,
        m_y: LengthAuto | InSteps[LengthAuto] = 0,
        row: bool | InSteps[bool] = False,
        reverse: bool | InSteps[bool] = False,
        flex_wrap: FlexWrap | InSteps[FlexWrap] = "nowrap",
        flex_grow: float | InSteps[float] = 0.0,
        flex_shrink: float | InSteps[float] = 1.0,
        align_items: AlignItemsSteps = "center",
        align_self: AlignItemsSteps = None,
        justify_self: AlignItemsSteps = None,
        align_content: AlignContentSteps = None,
        justify_content: AlignContentSteps = "center",
        gap: tuple[Length, Length] | InSteps[tuple[Length, Length]] = (0.0, 0.0),
        bg_color: str | None | InSteps[str | None] = None,
        url: None | str | InSteps[None | str] = None,
        name: str = "",
        debug_layout: bool | str | None = None,
        replace_steps: dict[int, int] | None = None,
        _content: NodeContent | InSteps[NodeContent] = None,
    ):
        """
        Create a new child box. See [Box reference](https://spirali.github.io/nelsie/guide/box/) for documentation
        """
        parent_box = self.get_box()
        if debug_layout is None:
            debug_layout = parent_box.slide.debug_layout
        else:
            debug_layout = parse_debug_layout(debug_layout)
        if z_level is None:
            z_level = parent_box.z_level

        if p_left is None:
            p_left = p_x
        if p_right is None:
            p_right = p_x
        if p_bottom is None:
            p_bottom = p_y
        if p_top is None:
            p_top = p_y

        if m_left is None:
            m_left = m_x
        if m_right is None:
            m_right = m_x
        if m_bottom is None:
            m_bottom = m_y
        if m_top is None:
            m_top = m_y

        deck = parent_box.deck
        box_id, node_id = deck._deck.new_box(
            deck.resources,
            parent_box.slide._slide_id,
            parent_box._box_id,
            active=_resolve_step_keywords(active, parent_box),
            show=_resolve_step_keywords(show, parent_box),
            z_level=z_level,
            x=x,
            y=y,
            width=width,
            height=height,
            border_radius=border_radius,
            p_left=p_left,
            p_right=p_right,
            p_top=p_top,
            p_bottom=p_bottom,
            m_left=m_left,
            m_right=m_right,
            m_top=m_top,
            m_bottom=m_bottom,
            row=row,
            reverse=reverse,
            flex_wrap=flex_wrap,
            flex_grow=flex_grow,
            flex_shrink=flex_shrink,
            align_items=align_items,
            align_self=align_self,
            justify_self=justify_self,
            align_content=align_content,
            justify_content=justify_content,
            gap=gap,
            bg_color=bg_color,
            url=url,
            name=name,
            debug_layout=debug_layout,
            replace_steps=replace_steps,
            content=_content,
        )
        return Box(deck, parent_box.slide, box_id, node_id, name, z_level)

    def overlay(self, **box_args):
        """
        Create a new box that spans over the box
        """
        box_args.setdefault("x", 0)
        box_args.setdefault("y", 0)
        box_args.setdefault("width", "100%")
        box_args.setdefault("height", "100%")
        return self.box(**box_args)

    def line_box(self, line_idx: int, **box_args):
        """
        Creates a new box over a text line in the box
        """
        return self.box(
            x=self.line_x(line_idx),
            y=self.line_y(line_idx),
            width=self.line_width(line_idx),
            height=self.line_height(line_idx),
            **box_args,
        )

    def text_anchor_box(self, anchor_id: int, **box_args):
        """
        Creates a new box over a text anchor in the box
        """

        return self.box(
            x=self.text_anchor_x(anchor_id),
            y=self.text_anchor_y(anchor_id),
            width=self.text_anchor_width(anchor_id),
            height=self.text_anchor_height(anchor_id),
            **box_args,
        )

    def x(self, width_fraction: float | int | None = None) -> LayoutExpr:
        """
        Get an expression with X coordinate relative to the box.
        """

        node_id = self.get_box().node_id
        expr = LayoutExpr.x(node_id)
        if width_fraction is None:
            return expr
        return expr + LayoutExpr.width(node_id, width_fraction)

    def y(self, height_fraction: float | int | None = None) -> LayoutExpr:
        """
        Get an expression with Y coordinate relative to the box.
        """
        node_id = self.get_box().node_id
        expr = LayoutExpr.y(node_id)
        if height_fraction is None:
            return expr
        return expr + LayoutExpr.height(node_id, height_fraction)

    def width(self, fraction: float | int = 1.0) -> LayoutExpr:
        """
        Get an expression with width of the parent box.
        """
        node_id = self.get_box().node_id
        return LayoutExpr.width(node_id, fraction)

    def height(self, fraction: float | int = 1.0) -> LayoutExpr:
        """
        Get an expression with height of the parent box.
        """
        node_id = self.get_box().node_id
        return LayoutExpr.height(node_id, fraction)

    def line_x(self, line_idx: int, width_fraction: float | int | None = None) -> LayoutExpr:
        """
        Get an expression with X coordinate of a given line of text in the box.
        """
        node_id = self.get_box().node_id
        expr = LayoutExpr.line_x(node_id, line_idx)
        if width_fraction is None:
            return expr
        return expr + LayoutExpr.line_width(node_id, line_idx, width_fraction)

    def line_y(self, line_idx: int, height_fraction: float | int | None = None) -> LayoutExpr:
        """
        Get an expression with Y coordinate of a given line of text in the box.
        """
        node_id = self.get_box().node_id
        expr = LayoutExpr.line_y(node_id, line_idx)
        if height_fraction is None:
            return expr
        return expr + LayoutExpr.line_height(node_id, line_idx, height_fraction)

    def line_width(self, line_idx: int, fraction: float | int = 1.0) -> LayoutExpr:
        """
        Get an expression with a width of a given line of text in the box.
        """
        node_id = self.get_box().node_id
        return LayoutExpr.line_width(node_id, line_idx, fraction)

    def line_height(self, line_idx: int, fraction: float | int = 1.0) -> LayoutExpr:
        """
        Get an expression with a height of a given line of text in the box.
        """
        node_id = self.get_box().node_id
        return LayoutExpr.line_height(node_id, line_idx, fraction)

    def text_anchor_x(self, anchor_id: int, width_fraction: float | int | None = None) -> LayoutExpr:
        """
        Get an expression with X coordinate of a given text anchor in the box.
        """
        node_id = self.get_box().node_id
        expr = LayoutExpr.text_anchor_x(node_id, anchor_id)
        if width_fraction is None:
            return expr
        return expr + LayoutExpr.text_anchor_width(node_id, anchor_id, width_fraction)

    def text_anchor_y(self, anchor_id: int, height_fraction: float | int | None = None) -> LayoutExpr:
        """
        Get an expression with Y coordinate of a given text anchor in the box.
        """
        node_id = self.get_box().node_id
        expr = LayoutExpr.text_anchor_y(node_id, anchor_id)
        if height_fraction is None:
            return expr
        return expr + LayoutExpr.text_anchor_height(node_id, anchor_id, height_fraction)

    def text_anchor_width(self, anchor_id: int, fraction: float | int = 1.0) -> LayoutExpr:
        """
        Get an expression with a height of a given text anchor in the box.
        """
        node_id = self.get_box().node_id
        return LayoutExpr.text_anchor_width(node_id, anchor_id, fraction)

    def text_anchor_height(self, anchor_id: int, fraction: float | int = 1.0) -> LayoutExpr:
        """
        Get an expression with a height of a given text anchor in the box.
        """
        node_id = self.get_box().node_id
        return LayoutExpr.text_anchor_height(node_id, anchor_id, fraction)


class Box(BoxBuilder):
    """
    The box in slide layout.

    It should be created via calling method `.box()` on a slide or another box.
    Note that interesting methods came from Box's parent: BoxBuilder.
    """

    def __init__(
        self,
        deck,
        slide,
        box_id,
        node_id,
        name: str,
        z_level: int,
    ):
        """
        @private
        """
        self.deck = deck
        self.slide = slide
        self._box_id = box_id
        self.node_id = node_id
        self.name = name
        self.z_level = z_level

    def get_box(self):
        """
        @private
        """
        return self
