class StepCounter:
    def __init__(self, init_value=1):
        self.step = init_value

    def increment(self, count=1):
        self.step += count

    def set(self, step):
        self.step = step

    def last(self):
        return self.step

    def last_p(self):
        return f"{self.step}+"

    def next(self):
        self.increment()
        return self.step

    def next_p(self):
        self.increment()
        return self.last_p()
