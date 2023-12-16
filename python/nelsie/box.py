import os
from dataclasses import dataclass

from .basictypes import (
    Length,
    LengthAuto,
    Position,
    Size,
    TextAlign,
    parse_debug_layout,
)
from .insteps import InSteps
from .shapes import Path
from .textstyle import TextStyle, _data_to_text_style

# class DrawChild:
#     def __init__(self, paths: InSteps[list[ExportPath]]):
#         self.paths = paths
#
#     def export(self):
#         return ExportDrawing(self.paths)


# BoxChild = Union[DrawChild, "Box"]


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


@dataclass
class BoxConfig:
    show: bool | str | InSteps[bool]
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
    bg_color: str | None | InSteps[str | None]
    name: str
    debug_layout: bool | None


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
        delimiters: str | None = "~{}",
        tab_width: int = 4,
        align: TextAlign = TextAlign.Start,
        **box_args,
    ):
        return self._text_box(
            text, style, None, delimiters, tab_width, box_args, align, None, None
        )

    def code(
        self,
        text: str,
        language: str,
        style: str | TextStyle | InSteps[TextStyle] | None = None,
        *,
        theme: str | None = None,
        delimiters: str | None = "~{}",
        tab_width: int = 4,
        align: TextAlign = TextAlign.Start,
        **box_args,
    ):
        if theme is None:
            theme = self.get_box().deck.default_theme
        return self._text_box(
            text, "code", style, None, tab_width, box_args, align, language, theme
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
    ):
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
        show: bool | str = True,
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
        bg_color: str | None | InSteps[str | None] = None,
        name: str = "",
        debug_layout: bool | None = None,
        _content: NodeContent | InSteps[NodeContent] = None,
    ):
        parent_box = self.get_box()
        debug_layout = parse_debug_layout(debug_layout)
        if z_level is None:
            z_level = parent_box.z_level
        config = BoxConfig(
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
            bg_color=bg_color,
            name=name,
            debug_layout=debug_layout,
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
