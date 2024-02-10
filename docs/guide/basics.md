
# Basics


## Slide deck

A presentation (a set of slides) is represented in Nelsie as an instance of SlideDeck class.

```python
from nelsie import SlideDeck

deck = SlideDeck()
```

In the constructor, you can define some values that will later be used as defaults for each slide; however, each slide can override these values individually. For example, you can define a default size for slides (if not specified, 1024x968 is used):

```python
deck = SlideDeck(width=1920, height=1080)
```

!!! note "Note on resolution"

    Since the main output format is PDF, which is a vector format, the resolution does not define the "quality" of the output, but mostly just the size ratio.


## Creating a new slide

You can create new slides in two ways, either using the `new_slide` method or using a decorator `slide`.

The example of using `new_slide`:

```python
slide = deck.new_slide(bg_color="blue")
slide.text("First slide")  # Put a text into slide
```

The example of using decorator `slide`:

```python
@deck.slide(bg_color="blue")
def first_slide(slide):
    slide.text("First slide")  # Put a text into slide
```

The decorator immediately calls the wrapped function that sets the content of the slide. The main function of the decorator is to break slides into individual functions for code clarity.


## Rendering the slide deck

Once all the slides have been created, you can render them into PDF with a `.render()` call on the slide deck.

```python
deck.render("slides.pdf")
```

You can also get SVG or PNG images from the slides, see the [Output formats section](output.md) for more information.