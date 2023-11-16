from .text.texttypes import StyledText
from .text.parse import parse_styled_text, export_styled_text
from .text.manager import TextStyleManager, TextStylesProviderMixin
from .text.textstyle import TextStyle
from .export import ExportNode, ExportStepValue
from .steps.stepsexport import export_step_value
from .parsers import parse_size, check_type_bool, parse_position
from .steps.insteps import InSteps, parse_steps
from .basictypes import Size, Position
from .colors import check_color


class BoxBuilder(TextStylesProviderMixin):
    def add_box(self, box: "Box"):
        raise NotImplementedError

    def get_slide(self):
        raise NotImplementedError

    def get_box(self):
        raise NotImplementedError

    def box(
        self,
        *,
        show: bool | str = True,
        x: Position | InSteps[Position] = None,
        y: Position | InSteps[Position] = None,
        width: Size | InSteps[Size] = "auto",
        height: Size | InSteps[Size] = "auto",
        row: bool | InSteps[bool] = False,
        reverse: bool | InSteps[bool] = False,
        bg_color: str | None | InSteps[str | None] = None,
        text: StyledText | None = None,
    ):
        parent_box = self.get_box()
        box = Box(
            slide=self.get_slide(),
            parent_id=parent_box.box_id if parent_box else None,
            style_manager=self.style_manager.copy(),
            show=show,
            x=x,
            y=y,
            width=width,
            height=height,
            bg_color=bg_color,
            row=row,
            reverse=reverse,
            text=text,
        )
        self.add_box(box)
        return box


class Box(BoxBuilder, TextStylesProviderMixin):
    def __init__(
        self,
        *,
        slide,
        parent_id: int | None,
        style_manager: TextStyleManager,
        show: bool | str,
        x: Position | InSteps[Position],
        y: Position | InSteps[Position],
        width: Size | InSteps[Size],
        height: Size | InSteps[Size],
        row: bool | InSteps[bool],
        reverse: bool | InSteps[bool],
        bg_color: str | None | InSteps[str | None],
        text: StyledText | None = None,
    ):
        self.slide = slide
        self.style_manager = style_manager

        show_steps = parse_steps(show)
        self.slide.update_min_steps(show_steps.n_steps)

        self.node = ExportNode(
            node_id=slide.new_box_id(),
            show=export_step_value(show_steps, self.slide),
            x=self._export_attr("x", x, lambda v: parse_position(parent_id, v, True)),
            y=self._export_attr("y", y, lambda v: parse_position(parent_id, v, False)),
            width=self._export_attr("width", width, parse_size),
            height=self._export_attr("height", height, parse_size),
            bg_color=self._export_attr("bg_color", bg_color, check_color),
            row=self._export_attr("row", row, check_type_bool),
            reverse=self._export_attr("reverse", reverse, check_type_bool),
            text=export_styled_text(text, self.slide)
            if text
            else export_step_value(None, self.slide),
        )
        self.children = []

    @property
    def box_id(self):
        return self.node.node_id

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

    def _text_box(self, text, style, delimiters, tab_width, box_args):
        text = text.replace("\t", " " * tab_width)
        if isinstance(style, str):
            style = self.style_manager.get_style(style)
        elif not isinstance(style, TextStyle):
            raise Exception("Invalid type for text style")
        parsed_text = parse_styled_text(text, delimiters, style, self.style_manager)
        return self.box(text=parsed_text, **box_args)

    def _export_attr(self, name, value, parser) -> ExportStepValue:
        try:
            return export_step_value(value, self.get_slide(), parser)
        except ValueError as e:
            raise ValueError(f"Invalid value for '{name}': {e}")

    def get_slide(self):
        return self.slide

    def get_box(self):
        return self

    def add_box(self, box: "Box"):
        self.children.append(box)

    def export(self) -> ExportNode:
        if self.children:
            self.node.children = [child.export() for child in self.children]
        return self.node
