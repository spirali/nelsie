from .steps import to_step_value

Size = float | str


class BoxBuilder:
    def add_box(self, box: "Box"):
        raise NotImplementedError

    def get_slide(self):
        raise NotImplementedError

    def box(self, *, width: Size, height: Size, bg_color: str | None = None):
        box = Box(slide=self.get_slide(), width=width, height=height, bg_color=bg_color)
        self.add_box(box)
        return box


class Box(BoxBuilder):
    def __init__(self, slide, width: Size, height: Size, bg_color: str | None):
        self.slide = slide
        self.width = width
        self.height = height
        self.bg_color = bg_color
        self.children = []

    def get_slide(self):
        return self.slide

    def add_box(self, box: "Box"):
        self.children.append(box)

    def render(self):
        result = {
            "width": to_step_value(self.width),
            "height": to_step_value(self.height),
            "bg_color": to_step_value(self.bg_color),
        }
        if not self.children:
            return result
        result["children"] = [child.render() for child in self.children]
        return result
