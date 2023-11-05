from .parsers import parse_size, parse_color, check_type
from .steps import to_step_value, StepValue

Size = int | float | str


class BoxBuilder:
    def add_box(self, box: "Box"):
        raise NotImplementedError

    def get_slide(self):
        raise NotImplementedError

    def box(
        self,
        *,
        width: Size | StepValue[Size] = "auto",
        height: Size | StepValue[Size] = "auto",
        row: bool | StepValue[bool] = False,
        reverse: bool | StepValue[bool] = False,
        bg_color: str | None | StepValue[str | None] = None,
    ):
        box = Box(
            slide=self.get_slide(),
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
        width: Size | StepValue[Size],
        height: Size | StepValue[Size],
        row: bool | StepValue[bool],
        reverse: bool | StepValue[bool],
        bg_color: str | None | StepValue[str | None],
    ):
        self.slide = slide
        self.width = to_step_value(width, parse_size)
        self.height = to_step_value(height, parse_size)
        self.bg_color = to_step_value(bg_color, parse_color)
        self.bg_color = to_step_value(bg_color, parse_color)
        self.row = to_step_value(row, lambda x: check_type(x, bool, "row"))
        self.reverse = to_step_value(reverse, lambda x: check_type(x, bool, "reverse"))
        self.children = []

    def get_slide(self):
        return self.slide

    def add_box(self, box: "Box"):
        self.children.append(box)

    def render(self):
        result = {
            "width": self.width,
            "height": self.height,
            "row": self.row,
            "reverse": self.reverse,
            "bg_color": self.bg_color,
        }
        if not self.children:
            return result
        result["children"] = [child.render() for child in self.children]
        return result
