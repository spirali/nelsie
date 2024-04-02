# Page counters

Page counters serves to show information about the current page or the total number of pages on slides.

The following variables can be used in any text on slide if parameter `parse_counters` of `.text()` (or `.code()`) is
set to `True`. The `<NAME>` is a name of the actual counter.

* `$(<NAME>_page)` -- The current page index (counted from 1) for the given counter.
* `$(<NAME>_pages)` -- The total number of pages of the counter.
* `$(<NAME>_slide)` -- The current slide index (counted from 1) for the given counter. Note that a slide may produce
  more steps and therefore may have more pages. For all pages of the same slide, the slide index remains the same if
  there is no inserted slided. In case of inserting a slide, switching back to the original slide also increases slide
  index counter if the inserted slide was included in counter.
* `$(<NAME>_slides)` -- The total number of slide for the given counter.

The counter `global` always exists and it includes all slides.

```nelsie

deck.set_style("default", TextStyle(size=80))

def show_counter(slide):
    slide.text("Page: $(global_page)/$(global_pages)",
               parse_counters=True,
               x=0, y=0, style=TextStyle(color="green"))

@deck.slide()
def counters_demo1(slide):
    show_counter(slide)
    slide.text("A1")
    slide.text("A2", show="2+")
    slide.text("A3", show="3+")

@deck.slide()
def counters_demo2(slide):
    show_counter(slide)
    slide.text("B1")
    slide.text("B2", show="2+")
```

# Custom counters

If you want to count only some slides, you can create a custom counter
and count only a subset of slides.

In the following code, we create a custom counter `"my"`:

```nelsie

deck.set_style("default", TextStyle(size=80))

def show_counter(slide):
    # Here we show values of counter "my"
    slide.text("Page: $(my_page)/$(my_pages)",
               parse_counters=True,
               x=0, y=0, style=TextStyle(color="green"))

@deck.slide(counters=["my"])  # Count this slide in counter "my"
def counters_demo1(slide):
    show_counter(slide)
    slide.text("A1")
    slide.text("A2", show="2+")
    slide.text("A3", show="3+")

@deck.slide()
def not_counted(slide):
    slide.text("NOT COUNTED")

@deck.slide(counters=["my"])  # Count this slide in counter "my"
def counters_demo2(slide):
    show_counter(slide)
    slide.text("B1")
    slide.text("B2", show="2+")
```