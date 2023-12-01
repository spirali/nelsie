from typing import Union
from dataclasses import dataclass

from .basictypes import Position, Size, Length, LengthAuto, parse_debug_layout
from .insteps import InSteps


# class DrawChild:
#     def __init__(self, paths: InSteps[list[ExportPath]]):
#         self.paths = paths
#
#     def export(self):
#         return ExportDrawing(self.paths)


# BoxChild = Union[DrawChild, "Box"]

@dataclass
class BoxConfig:
    show: bool | str
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
    #content: NodeContent | InSteps[NodeContent] = None,
    name: str
    debug_layout: bool | None


class BoxBuilder:

    def get_box(self):
        raise NotImplementedError

    def image(self, filename: str, enable_steps=True, shift_steps=0, **box_args):
        """
        Load image; supported formats: svg, png, jpeg, gif, ora
        """
        assert shift_steps >= 0
        slide = self.get_slide()
        if slide.image_directory is not None:
            filename = os.path.join(slide.image_directory, filename)
        image = Image(
            filename=os.path.abspath(filename),
            enable_steps=enable_steps,
            shift_steps=shift_steps,
        )
        return self.box(content=image, **box_args)

    """
    def text(
            self,
            text: str,
            *,
            style: str | TextStyle = "default",
            delimiters: str | None = "~{}",
            tab_width: int = 4,
            **box_args,
    ):
        return self._text_box(text, style, delimiters, tab_width, box_args)

    def draw(self, paths: Path | list[Path] | InSteps[Path | list[Path]]):
        paths = to_steps(paths)
        paths = paths.map(lambda p: [p] if not isinstance(p, list) else p)
        print(paths.in_step_values)
        slide = self.get_slide()
        box_id = self.get_box().box_id
        export_paths = export_step_value(
            paths, slide, lambda p: [path.export(box_id) for path in p], default=[]
        )
        self.add_child(DrawChild(export_paths))

    def _text_box(self, text, style, delimiters, tab_width, box_args):
        text = text.replace("\t", " " * tab_width)
        default = self.style_manager.get_style("default")

        if style == "default":
            style = default
        if isinstance(style, str):
            style = default.update(self.style_manager.get_style(style))
        elif isinstance(style, TextStyle):
            style = default.update(style)
        else:
            raise Exception("Invalid type for text style")
        parsed_text = parse_styled_text(text, delimiters, style, self.style_manager)
        return self.box(content=parsed_text, **box_args)
    """

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
            # content: NodeContent | InSteps[NodeContent] = None,
            name: str = "",
            debug_layout: bool | None = None,
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
            debug_layout=debug_layout
        )
        box_id, node_id = parent_box._deck.new_box(parent_box.slide._slide_id, parent_box._box_id, config)
        box = Box(parent_box._deck, parent_box.slide, box_id, node_id, name, z_level)
        return box


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
        self._deck = deck
        self.slide = slide
        self._box_id = box_id
        self.node_id = node_id
        self.name = name
        self.z_level = z_level

    def get_box(self):
        return self