from .parsers import parse_size, parse_color, check_type, check_type_bool
from .steps import process_step_value, InSteps, process_step_bool_def

Size = int | float | str


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

        self.show, n_steps = process_step_bool_def(show)
        self.slide.update_min_steps(n_steps)

        self._set_attr("width", width, parse_size)
        self._set_attr("height", height, parse_size)
        self._set_attr("bg_color", bg_color, parse_color)
        self._set_attr("row", row, check_type_bool)
        self._set_attr("reverse", reverse, check_type_bool)
        self.children = []

    def _set_attr(self, name, value, parser=None):
        try:
            result, n_steps = process_step_value(value, parser)
        except ValueError as e:
            raise ValueError(f"Invalid value for '{name}': {e}")
        self.slide.update_min_steps(n_steps)
        setattr(self, name, result)

    def get_slide(self):
        return self.slide

    def add_box(self, box: "Box"):
        self.children.append(box)

    def render(self):
        result = {
            "width": self.width,
            "height": self.height,
            "show": self.show,
            "row": self.row,
            "reverse": self.reverse,
            "bg_color": self.bg_color,
        }
        if not self.children:
            return result
        result["children"] = [child.render() for child in self.children]
        return result
