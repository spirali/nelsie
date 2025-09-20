from typing import Callable

from ..box import Box, BoxBuilderMixin

DEFAULT_BULLETS = ("•", "⁃", "‣")


class ListBox(BoxBuilderMixin):
    def __init__(
        self,
        parent_box: Box,
        list_type: str | Callable = "unordered",
        style="default",
        indent_size=15,
        initial_counter_value=1,
        _counters=None,
    ):
        self.main_box = parent_box.box(align_items="start")
        self.style = style
        self.indent_size = indent_size
        self.list_type = list_type

        if _counters is None:
            self.counters = [initial_counter_value]
        else:
            self.counters = _counters[:] + [initial_counter_value]

    @property
    def level(self):
        return len(self.counters) - 1

    def add(self, item):
        self.main_box.add(item)

    def create_item_boxes(self, box_args) -> (Box, Box):
        item_box = self.main_box.box(row=True, align_items="start")
        show = box_args.get("show", True)
        active = box_args.get("active", True)
        box1 = item_box.box(
            row=True,
            width=self.indent_size,
            justify_content="start",
            show=show,
            active=active,
        )
        box2 = item_box.box(**box_args)
        return box1, box2

    def box(self, **box_args) -> Box:
        counter = self.counters[-1]
        self.counters[-1] += 1
        box1, box2 = self.create_item_boxes(box_args)
        if isinstance(self.list_type, Callable):
            self.list_type(box1)
        elif self.list_type == "unordered":
            box1.text(
                DEFAULT_BULLETS[self.level % len(DEFAULT_BULLETS)], style=self.style
            )
        elif self.list_type == "1":
            box1.text(f"{counter}.", style=self.style)
        elif self.list_type == "a":
            box1.text(f"{chr(ord('a') - 1 + counter)}.", style=self.style)
        elif self.list_type == "A":
            box1.text(f"{chr(ord('A') - 1 + counter)}.", style=self.style)
        return box2

    def list(self, list_type="unordered", **box_args):
        box1, box2 = self.create_item_boxes(box_args)
        return ListBox(box2, list_type=list_type, _counters=self.counters)
