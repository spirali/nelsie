# Postprocessing

Each slide may have attached a postprocessing function that is called after
are all steps are resolved, so you may do some last-minute modification of the slides
before rendering.

The postprocessing has to return either *the original unchanged slide* or *a modified copy of the slide*.

```nelsie
def do_postprocess(slide, current, total):
    slide = slide.copy()
    slide.add(Rect(Point(0, 0), Point(100, 100), fill_color="red"))
    return slide

@deck.slide(postprocess_fn=do_postprocess)
def counters_demo1(slide):
    slide.text("A1")
    slide.text("A2", show="2+")
    slide.text("A3", show="3+")
```

# Page counters

Page counters serves to show information about the current page or the total number of pages on slides. Counters are passed into the postprocessing function and
contains information about position of each slide.

The is always a counter `global` that automatically includes all slides.

```nelsie
deck.set_style("default", TextStyle(size=80))

def add_page_info(slide, current, total):
    slide = slide.copy()

    n_slide = current["global"].slide
    n_slide_total = total["global"].slide

    n_page = current["global"].page
    n_page_total = total["global"].page

    slide.text(
        f"Slide: {n_slide}/{n_slide_total}\nPage: {n_page}/{n_page_total}",
        bg_color="gray", x = 0, y = 0
    )
    return slide


@deck.slide(postprocess_fn=add_page_info)
def counters_demo1(slide):
    slide.text("A1")
    slide.text("A2", show="2+")
    slide.text("A3", show="3+")

@deck.slide(postprocess_fn=add_page_info)
def counters_demo2(slide):
    slide.text("B1")
    slide.text("B2", show="2+")
```

## Custom counters

If you want to count only some slides, you can create a custom counter
and count only a subset of slides.

In the following code, we create a custom counter `"my"`:

```nelsie
deck.set_style("default", TextStyle(size=80))

def add_page_info(slide, current, total):
    slide = slide.copy()

    n_slide = current["my"].slide
    n_slide_total = total["my"].slide

    n_page = current["my"].page
    n_page_total = total["my"].page

    slide.text(
        f"Slide: {n_slide}/{n_slide_total}\nPage: {n_page}/{n_page_total}",
        bg_color="gray", x = 0, y = 0
    )
    return slide

# Count this slide in counter "my"
@deck.slide(counters=["my"], postprocess_fn=add_page_info)
def counters_demo1(slide):
    slide.text("A1")
    slide.text("A2", show="2+")
    slide.text("A3", show="3+")

# Do not include this slide in counter "my"
@deck.slide(postprocess_fn=add_page_info)
def not_counted(slide):
    slide.text("NOT COUNTED")

# Count this slide in counter "my"
@deck.slide(counters=["my"], postprocess_fn=add_page_info)
def counters_demo2(slide):
    slide.text("B1")
    slide.text("B2", show="2+")
```
