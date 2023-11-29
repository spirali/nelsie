import os
from typing import Union

from .basictypes import Position, Size, Length, LengthAuto
from .colors import check_color
from .export import (
    ExportDrawing,
    ExportNode,
    ExportPath,
    ExportStepValue,
    Image,
    NodeContent,
)
from .parsers import check_type_bool, parse_position, parse_size, parse_length, parse_length_auto
from .shapes import Path
from .steps.insteps import InSteps, parse_steps, to_steps
from .steps.stepsexport import export_step_value
from .text.manager import TextStyleManager, TextStylesProviderMixin
from .text.parse import parse_styled_text
from .text.textstyle import TextStyle


class DrawChild:
    def __init__(self, paths: InSteps[list[ExportPath]]):
        self.paths = paths

    def export(self):
        return ExportDrawing(self.paths)


BoxChild = Union[DrawChild, "Box"]


class BoxBuilder(TextStylesProviderMixin):
    def add_child(self, child: BoxChild):
        raise NotImplementedError

    def get_slide(self):
        raise NotImplementedError

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
        print(paths.values)
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
        content: NodeContent | InSteps[NodeContent] = None,
        name: str = "",
        debug_layout: bool | None = None,
    ):
        parent_box = self.get_box()
        if z_level is None:
            z_level = parent_box.z_level
        box = Box(
            slide=self.get_slide(),
            parent_id=parent_box.box_id if parent_box else None,
            style_manager=self.style_manager.copy(),
            show=show,
            z_level=z_level,
            x=x,
            y=y,
            width=width,
            height=height,
            bg_color=bg_color,
            p_left=p_left,
            p_right=p_right,
            p_bottom=p_bottom,
            p_top=p_top,
            m_left=m_left,
            m_right=m_right,
            m_top=m_top,
            m_bottom=m_bottom,
            row=row,
            reverse=reverse,
            content=content,
            name=name,
            debug_layout=debug_layout,
        )
        self.add_child(box)
        return box


class Box(BoxBuilder, TextStylesProviderMixin):
    def __init__(
        self,
        *,
        slide,
        parent_id: int | None,
        style_manager: TextStyleManager,
        show: bool | str,
        z_level: int | InSteps[int],
        x: Position | InSteps[Position],
        y: Position | InSteps[Position],
        width: Size | InSteps[Size],
        height: Size | InSteps[Size],
        p_left: Length | InSteps[Length],
        p_right: Length | InSteps[Length],
        p_top: Length | InSteps[Length],
        p_bottom: Length | InSteps[Length],
        m_left: LengthAuto | InSteps[LengthAuto],
        m_right: LengthAuto | InSteps[LengthAuto],
        m_top: LengthAuto | InSteps[LengthAuto],
        m_bottom: LengthAuto | InSteps[LengthAuto],
        row: bool | InSteps[bool],
        reverse: bool | InSteps[bool],
        bg_color: str | None | InSteps[str | None],
        content: NodeContent | InSteps[NodeContent] = None,
        name: str,
        debug_layout: bool | str | None,
    ):
        self.slide = slide
        self.style_manager = style_manager
        self.z_level = z_level

        show_steps = parse_steps(show)
        self.slide.update_min_steps(show_steps.n_steps)

        if debug_layout is None:
            debug_layout = slide.debug_layout
        if not debug_layout:
            debug_layout = None
        elif debug_layout is True:  # Exactly True!
            debug_layout = "#FF00FF"
        self.node = ExportNode(
            node_id=slide.new_box_id(),
            show=export_step_value(show_steps, self.slide),
            z_level=export_step_value(z_level, self.slide),
            x=self._export_attr("x", x, lambda v: parse_position(parent_id, v, True)),
            y=self._export_attr("y", y, lambda v: parse_position(parent_id, v, False)),
            width=self._export_attr("width", width, parse_size),
            height=self._export_attr("height", height, parse_size),
            p_left=self._export_attr("p_left", p_left, parse_length),
            p_right=self._export_attr("p_right", p_right, parse_length),
            p_top=self._export_attr("p_top", p_top, parse_length),
            p_bottom=self._export_attr("p_bottom", p_bottom, parse_length),
            m_left=self._export_attr("m_left", m_left, parse_length_auto),
            m_right=self._export_attr("m_right", m_right, parse_length_auto),
            m_top=self._export_attr("m_top", m_top, parse_length_auto),
            m_bottom=self._export_attr("m_bottom", m_bottom, parse_length_auto),
            bg_color=self._export_attr("bg_color", bg_color, check_color),
            row=self._export_attr("row", row, check_type_bool),
            reverse=self._export_attr("reverse", reverse, check_type_bool),
            content=export_step_value(content, self.slide),
            debug_layout=debug_layout,
            name=name,
        )
        self.children: list[BoxChild] = []

    @property
    def box_id(self):
        return self.node.node_id

    def _export_attr(self, name, value, parser) -> ExportStepValue:
        try:
            return export_step_value(value, self.get_slide(), parser)
        except ValueError as e:
            raise ValueError(f"Invalid value for '{name}': {e}")

    def get_slide(self):
        return self.slide

    def get_box(self):
        return self

    def add_child(self, child: BoxChild):
        self.children.append(child)

    def export(self) -> ExportNode:
        self.node.children = [child.export() for child in self.children]
        return self.node
