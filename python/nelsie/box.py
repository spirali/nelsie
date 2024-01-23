import os
from dataclasses import dataclass

from .basictypes import (
    Align,
    FlexWrap,
    Length,
    LengthAuto,
    Position,
    Size,
    TextAlign,
    parse_debug_layout,
)
from .insteps import InSteps
from .layoutexpr import LayoutExpr
from .shapes import Path
from .textstyle import TextStyle, _data_to_text_style


@dataclass
class TextContent:
    text: str
    style1: TextStyle | str | None
    style2: TextStyle | str | None
    formatting_delimiters: str
    text_align: TextAlign
    syntax_language: str | None
    syntax_theme: str | None


@dataclass
class ImageContent:
    path: str
    enable_steps: bool
    shift_steps: int


NodeContent = ImageContent | TextContent | None
AlignSteps = Align | None | InSteps[Align | None]


@dataclass
class BoxConfig:
    active: bool | str | int | InSteps[bool]
    show: bool | str | int | InSteps[bool]
    z_level: int | InSteps[int] | None
    x: Position | InSteps[Position]
    y: Position | InSteps[Position]
    width: Size | InSteps[Size]
    height: Size | InSteps[Size]
    p_left: Length | InSteps[Length]
    p_right: Length | InSteps[Length]
    p_top: Length | InSteps[Length]
    p_bottom: Length | InSteps[Length]
    m_left: LengthAuto | InSteps[LengthAuto]
    m_right: LengthAuto | InSteps[LengthAuto]
    m_top: LengthAuto | InSteps[LengthAuto]
    m_bottom: LengthAuto | InSteps[LengthAuto]
    row: bool | InSteps[bool]
    reverse: bool | InSteps[bool]
    flex_wrap: FlexWrap | InSteps[FlexWrap]
    flex_grow: float | InSteps[float]
    flex_shrink: float | InSteps[float]
    align_items: AlignSteps
    align_self: AlignSteps
    justify_self: AlignSteps
    align_content: AlignSteps
    justify_content: AlignSteps
    gap: tuple[Length, Length] | InSteps[tuple[Length, Length]]
    bg_color: str | None | InSteps[str | None]
    name: str
    debug_layout: bool | None
    replace_steps: dict[int, int] | None


class BoxBuilder:
    def get_box(self):
        raise NotImplementedError

    def set_style(self, name: str, style: TextStyle):
        box = self.get_box()
        deck = box.deck
        deck._deck.set_style(
            deck.resources, name, style, False, box.slide._slide_id, box._box_id
        )

    def update_style(self, name: str, style: TextStyle):
        box = self.get_box()
        deck = box.deck
        deck._deck.set_style(
            deck.resources, name, style, True, box.slide._slide_id, box._box_id
        )

    def get_style(self, name: str, step: int = 1) -> TextStyle:
        box = self.get_box()
        return _data_to_text_style(
            box.deck._deck.get_style(name, step, box.slide._slide_id, box._box_id)
        )

    def image(self, path: str, enable_steps=True, shift_steps=0, **box_args):
        """
        Load image; supported formats: svg, png, jpeg, gif, ora
        """
        assert shift_steps >= 0
        slide = self.get_box().slide
        if slide.image_directory is not None:
            path = os.path.join(slide.image_directory, path)
        image = ImageContent(
            path=os.path.abspath(path),
            enable_steps=enable_steps,
            shift_steps=shift_steps,
        )
        return self.box(_content=image, **box_args)

    def text(
        self,
        text: str,
        style: str | TextStyle | InSteps[TextStyle] | None = None,
        *,
        parse_styles: bool = True,
        delimiters: str | None = "~{}",
        tab_width: int = 4,
        align: TextAlign = TextAlign.Start,
        strip=True,
        **box_args,
    ):
        return self._text_box(
            text,
            style,
            None,
            delimiters if parse_styles else None,
            tab_width,
            box_args,
            align,
            None,
            None,
            strip,
        )

    def code(
        self,
        text: str,
        language: str,
        style: str | TextStyle | InSteps[TextStyle] | None = None,
        *,
        theme: str | None = None,
        parse_styles: bool = False,
        delimiters: str | None = "~{}",
        tab_width: int = 4,
        align: TextAlign = TextAlign.Start,
        strip=True,
        **box_args,
    ):
        if theme is None:
            theme = self.get_box().deck.default_theme
        return self._text_box(
            text,
            "code",
            style,
            delimiters if parse_styles else None,
            tab_width,
            box_args,
            align,
            language,
            theme,
            strip,
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
    ):
        if strip:
            text = text.strip()
        text = text.replace("\t", " " * tab_width)
        text_content = TextContent(
            text=text,
            style1=style1,
            style2=style2,
            formatting_delimiters=delimiters,
            text_align=align,
            syntax_language=language,
            syntax_theme=theme,
        )
        return self.box(_content=text_content, **box_args)

    def draw(self, paths: Path | list[Path] | InSteps[Path | list[Path]]):
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
        p_left: Length | InSteps[Length] = 0,
        p_right: Length | InSteps[Length] = 0,
        p_top: Length | InSteps[Length] = 0,
        p_bottom: Length | InSteps[Length] = 0,
        m_left: LengthAuto | InSteps[LengthAuto] = 0,
        m_right: LengthAuto | InSteps[LengthAuto] = 0,
        m_top: LengthAuto | InSteps[LengthAuto] = 0,
        m_bottom: LengthAuto | InSteps[LengthAuto] = 0,
        row: bool | InSteps[bool] = False,
        reverse: bool | InSteps[bool] = False,
        flex_wrap: FlexWrap | InSteps[FlexWrap] = FlexWrap.NoWrap,
        flex_grow: float | InSteps[float] = 0.0,
        flex_shrink: float | InSteps[float] = 1.0,
        align_items: AlignSteps = Align.Center,
        align_self: AlignSteps = None,
        justify_self: AlignSteps = None,
        align_content: AlignSteps = None,
        justify_content: AlignSteps = Align.Center,
        gap: tuple[Length, Length] | InSteps[tuple[Length, Length]] = (0.0, 0.0),
        bg_color: str | None | InSteps[str | None] = None,
        name: str = "",
        debug_layout: bool | None = None,
        replace_steps: dict[int, int] | None = None,
        _content: NodeContent | InSteps[NodeContent] = None,
    ):
        parent_box = self.get_box()
        debug_layout = parse_debug_layout(debug_layout)
        if z_level is None:
            z_level = parent_box.z_level
        config = BoxConfig(
            active=active,
            show=show,
            z_level=z_level,
            x=x,
            y=y,
            width=width,
            height=height,
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
            name=name,
            debug_layout=debug_layout,
            replace_steps=replace_steps,
        )
        deck = parent_box.deck
        box_id, node_id = deck._deck.new_box(
            deck.resources,
            parent_box.slide._slide_id,
            parent_box._box_id,
            config,
            _content,
        )
        return Box(deck, parent_box.slide, box_id, node_id, name, z_level)

    def overlay(self, **box_args):
        box_args.setdefault("x", 0)
        box_args.setdefault("y", 0)
        box_args.setdefault("width", "100%")
        box_args.setdefault("height", "100%")
        return self.box(**box_args)

    def line_box(self, line_idx: int, **box_args):
        return self.box(
            x=self.line_x(line_idx),
            y=self.line_y(line_idx),
            width=self.line_width(line_idx),
            height=self.line_height(line_idx),
            **box_args,
        )

    def text_anchor_box(self, anchor_id: int, **box_args):
        return self.box(
            x=self.text_anchor_x(anchor_id),
            y=self.text_anchor_y(anchor_id),
            width=self.text_anchor_width(anchor_id),
            height=self.text_anchor_height(anchor_id),
            **box_args,
        )

    def x(self, width_fraction: float | int | None = None):
        node_id = self.get_box().node_id
        expr = LayoutExpr.x(node_id)
        if width_fraction is None:
            return expr
        return expr + LayoutExpr.width(node_id, width_fraction)

    def y(self, height_fraction: float | int | None = None):
        node_id = self.get_box().node_id
        expr = LayoutExpr.y(node_id)
        if height_fraction is None:
            return expr
        return expr + LayoutExpr.height(node_id, height_fraction)

    def width(self, fraction: float | int = 1.0):
        node_id = self.get_box().node_id
        return LayoutExpr.width(node_id, fraction)

    def height(self, fraction: float | int = 1.0):
        node_id = self.get_box().node_id
        return LayoutExpr.height(node_id, fraction)

    def line_x(self, line_idx: int, width_fraction: float | int | None = None):
        node_id = self.get_box().node_id
        expr = LayoutExpr.line_x(node_id, line_idx)
        if width_fraction is None:
            return expr
        return expr + LayoutExpr.line_width(node_id, line_idx, width_fraction)

    def line_y(self, line_idx: int, height_fraction: float | int | None = None):
        node_id = self.get_box().node_id
        expr = LayoutExpr.line_y(node_id, line_idx)
        if height_fraction is None:
            return expr
        return expr + LayoutExpr.line_height(node_id, line_idx, height_fraction)

    def line_width(self, line_idx: int, fraction: float | int = 1.0):
        node_id = self.get_box().node_id
        return LayoutExpr.line_width(node_id, line_idx, fraction)

    def line_height(self, line_idx: int, fraction: float | int = 1.0):
        node_id = self.get_box().node_id
        return LayoutExpr.line_height(node_id, line_idx, fraction)

    def text_anchor_x(self, anchor_id: int, width_fraction: float | int | None = None):
        node_id = self.get_box().node_id
        expr = LayoutExpr.text_anchor_x(node_id, anchor_id)
        if width_fraction is None:
            return expr
        return expr + LayoutExpr.text_anchor_width(node_id, anchor_id, width_fraction)

    def text_anchor_y(self, anchor_id: int, height_fraction: float | int | None = None):
        node_id = self.get_box().node_id
        expr = LayoutExpr.text_anchor_y(node_id, anchor_id)
        if height_fraction is None:
            return expr
        return expr + LayoutExpr.text_anchor_height(node_id, anchor_id, height_fraction)

    def text_anchor_width(self, anchor_id: int, fraction: float | int = 1.0):
        node_id = self.get_box().node_id
        return LayoutExpr.text_anchor_width(node_id, anchor_id, fraction)

    def text_anchor_height(self, anchor_id: int, fraction: float | int = 1.0):
        node_id = self.get_box().node_id
        return LayoutExpr.text_anchor_height(node_id, anchor_id, fraction)


class Box(BoxBuilder):
    def __init__(
        self,
        deck,
        slide,
        box_id,
        node_id,
        name: str,
        z_level: int,
    ):
        self.deck = deck
        self.slide = slide
        self._box_id = box_id
        self.node_id = node_id
        self.name = name
        self.z_level = z_level

    def get_box(self):
        return self
