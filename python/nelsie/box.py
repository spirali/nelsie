from .export import ExportNode, ExportStepValue
from .parsers import parse_size, check_color, check_type, check_type_bool
from .steps import InSteps, parse_steps, to_steps, export_step_value
from .basictypes import Size


class BoxBuilder:
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
    ):
        box = Box(
            slide=self.get_slide(),
            show=show,
            width=width,
            height=height,
            bg_color=bg_color,
            row=row,
            reverse=reverse,
        )
        self.add_box(box)
        return box


class Box(BoxBuilder):
    def __init__(
        self,
        slide,
        *,
        show: bool | str,
        width: Size | InSteps[Size],
        height: Size | InSteps[Size],
        row: bool | InSteps[bool],
        reverse: bool | InSteps[bool],
        bg_color: str | None | InSteps[str | None],
    ):
        self.slide = slide

        show_steps = parse_steps(show)
        self.slide.update_min_steps(show_steps.n_steps)

        self.node = ExportNode(
            show=export_step_value(show_steps, self.slide),
            width=self._export_attr("width", width, parse_size),
            height=self._export_attr("height", height, parse_size),
            bg_color=self._export_attr("bg_color", bg_color, check_color),
            row=self._export_attr("row", row, check_type_bool),
            reverse=self._export_attr("reverse", reverse, check_type_bool),
        )
        self.children = []

    def _export_attr(self, name, value, parser) -> ExportStepValue:
        try:
            return export_step_value(value, self.slide, parser)
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
