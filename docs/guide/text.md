# Text

This section is about drawing text on slides.

Text is drawn on a box (or slide) by calling the `.text()` method. It creates a new box containing a text.

```nelsie

@deck.slide()
def text_demo(slide):
    slide.text("Hello world!")
```

!!! note "Note for Elsie users"

    Calling `.text()` creates a new box; this is a different behavior than in Elsie, where calling `.text()` does not create a new box, which very often leads to code like `.box().text()` to create a wrapping box. This is not necessary in Nelsie.


## Text styles


## Named styles


## Inline styles


## 