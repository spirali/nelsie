from .text.texttypes import StyledText
from .text.parse import parse_styled_text, export_styled_text
from .text.manager import TextStyleManager, TextStylesProviderMixin
from .text.textstyle import TextStyle
from .export import ExportNode, ExportStepValue
from .steps.stepsexport import export_step_value
from .parsers import parse_size, check_type_bool
from .steps.insteps import InSteps, parse_steps
from .basictypes import Size
from .colors import check_color


class BoxBuilder(TextStylesProviderMixin):
    def add_box(self, box: "Box"):
        raise NotImplementedError

    def get_slide(self):
        raise NotImplementedError

    def box(
        self,
        *,
        show: bool | str = True,
        width: Size | InSteps[Size] = "auto",
        height: Size | InSteps[Size] = "auto",
        row: bool | InSteps[bool] = False,
        reverse: bool | InSteps[bool] = False,
        bg_color: str | None | InSteps[str | None] = None,
        text: StyledText | None = None,
    ):
        box = Box(
            slide=self.get_slide(),
            style_manager=self.style_manager.copy(),
            show=show,
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
        style_manager: TextStyleManager,
        show: bool | str,
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

        print("!!!!", text)

        self.node = ExportNode(
            show=export_step_value(show_steps, self.slide),
            width=self._export_attr("width", width, parse_size),
            height=self._export_attr("height", height, parse_size),
            bg_color=self._export_attr("bg_color", bg_color, check_color),
            row=self._export_attr("row", row, check_type_bool),
            reverse=self._export_attr("reverse", reverse, check_type_bool),
            text=export_step_value(
                text,
                self.get_slide(),
                lambda t: export_styled_text(self.slide, t) if t is not None else None,
            ),
        )
        print(">>>", self.node.text)
        self.children = []

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

    def add_box(self, box: "Box"):
        self.children.append(box)

    def export(self) -> ExportNode:
        if self.children:
            self.node.children = [child.export() for child in self.children]
        return self.node
