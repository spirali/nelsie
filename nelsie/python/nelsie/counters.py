from dataclasses import dataclass
from copy import deepcopy

@dataclass
class PageCounter:
    slide: int = 0
    page: int = 0


class CounterStorage:

    def __init__(self):
        self.global_counter = PageCounter()
        self.counters = {"global": self.global_counter}
        self.registered_texts = []

    def increment_slide(self):
        self.global_counter.slide += 1

    def increment_page(self):
        self.global_counter.page += 1

    def register_text(self, text):
        self.registered_texts.append((text, deepcopy(self.global_counter)))
