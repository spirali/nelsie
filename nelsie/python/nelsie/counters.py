from dataclasses import dataclass


@dataclass
class PageCounter:
    slide: int = 0
    page: int = 0


class CounterStorage:
    def __init__(self):
        self.global_counter = PageCounter()
        self.counters = {"global": self.global_counter}
        self.registered_texts = []

    def increment_slide(self, counters):
        if counters is not None:
            for counter in counters:
                if counter not in self.counters:
                    self.counters[counter] = PageCounter()
                self.counters[counter].slide += 1
        self.global_counter.slide += 1

    def increment_page(self, counters, count=1):
        if counters is not None:
            for counter in counters:
                if counter not in self.counters:
                    self.counters[counter] = PageCounter()
                self.counters[counter].page += count
        self.global_counter.page += count

    def __getitem__(self, item):
        count = self.counters.get(item)
        if count is None:
            return PageCounter()
        return count
