# Slide insertion

Slide insertion serves for inserting a new slide in a sequence of steps of another slide.

The slide insertion is implemented as methods `.new_slide_at()` and `.slide_at()` on a slide (not slide deck!).
These methods are equivalent to `.new_slide` and `.slide` on a slide deck, but it takes one mandatory argument:
a step of the parent slide where it should be inserted.
Inserted slides are completely independent slides with its own step counters.

## Basic example of a slide insertion

```nelsie
deck.set_style("default", TextStyle(size=80))

@deck.slide()
def parent_slide(slide):
    slide.text("Step 1")
    slide.text("Step 2", show="2+")
    slide.text("Step 3", show="3+")

@parent_slide.slide_at(3)
def inserted_slide(slide):
    slide.text("Insertion!", TextStyle(color="red"))
```

## Insertion to the same place

When more slides are inserted to the same place, they are placed in the same order as
they have been inserted:

```nelsie
deck.set_style("default", TextStyle(size=80))

@deck.slide()
def parent_slide(slide):
    slide.text("Step 1")
    slide.text("Step 2", show="2+")
    slide.text("Step 3", show="3+")

@parent_slide.slide_at(3)
def inserted_slide(slide):
    slide.text("Insertion 1", TextStyle(color="red"))

@parent_slide.slide_at(3)
def inserted_slide(slide):
    slide.text("Insertion 2", TextStyle(color="blue"))
```

## Steps and insertions into inserted slides

Inserted slides may also have steps and inserted slides.

```nelsie
deck.set_style("default", TextStyle(size=80))

@deck.slide()
def parent_slide(slide):
    slide.text("Step 1")
    slide.text("Step 2", show="2+")
    slide.text("Step 3", show="3+")

@parent_slide.slide_at(3)
def inserted_slide(slide):
    slide.set_style("default", TextStyle(color="red"))
    slide.text("Insertion: step 1")
    slide.text("Insertion: step 2", show="2+")
    slide.text("Insertion: step 3", show="3+")

@inserted_slide.slide_at(2)
def inserted_slide(slide):
    slide.text("Sub insertion", TextStyle(color="blue"))
```