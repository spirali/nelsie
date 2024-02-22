# Layout expressions

Layout expressions are used to define positions and sizes in relation to
other elements of slides at a time when the layout has not yet been calculated.
Layout expressions is a placeholder that gets its value when the element on which the expression depends is
calculated.

Layout expressions can be used in definition of position and size of boxes and points on paths.

There are three of type of layout expressions:

* **Box layout expressions**: They refer to a box layout properties. They are created by calling `.x()`, `.y()`, `.width()`, `.height()` on a box coordines and width.
* **Line expressions**: They refer to a line of a text. They are created by calling `.line_x(line_idx)`, `.line_y(line_idx)`, `.line_width(line_idx)`, `.line_height(line_idx)` on a box with a text.
* **Text anchor expressions**: They refer to a part of a text. They are created by calling `.text_anchor_x(anchor_id)`, `.text_anchor_y(anchor_id)`, `.text_anchor_width(anchor_id)`, `.text_anchor_height(anchor_id)` on a box with a text.

## Box layout expressions

```nelsie
@deck.slide()
def layout_expression_demo(slide):
    box1 = slide.box(width=300, height=100, bg_color="red")
    box2 = slide.box(width=150, height=200, bg_color="green")

    # Create a new box relative to the box1 x-position and box2 y-position
    slide.box(x=box1.x(), y=box2.y(), width=50, height=50, bg_color="blue")
```

## Line layout expressions

Line layout expressions require one parameter that is the index of the line; the index is counted from zero.

```nelsie
@deck.slide()
def layout_expression_demo(slide):
    box = slide.text("""
Lorem ipsum dolor sit amet,
consectetuer adipiscing elit.
Nulla turpis magna, cursus sit amet, suscipit a,
interdum id, felis.
    """)

    slide.box(x=box.line_x(1),
              y=box.line_y(1),
              width=box.line_width(1),
              height=box.line_height(1),
              bg_color="lightgreen",
              z_level=-1)  # Set z-level to draw the box below the text
```

## Text anchor layout expressions

Text anchor is a part of a text. Each text anchor has an unsigned integer ID that has to be
unique within the text. Text anchor is defined in the same syntax as text styles, except that
the name of style is composed of digits. These digits then define the ID of the anchor.


```nelsie
@deck.slide()
def layout_expression_demo(slide):
    box = slide.text("""
Lorem ~42{ipsum} dolor sit amet,
consectetuer adipiscing elit.
~105{Nulla turpis magna}, cursus sit amet, suscipit a,
interdum id, felis.
    """)

    slide.box(x=box.text_anchor_x(42),
              y=box.text_anchor_y(42),
              width=box.text_anchor_width(42),
              height=box.text_anchor_height(42),
              bg_color="lightgreen",
              z_level=-1)  # Set z-level to draw the box below the text

    slide.box(x=box.text_anchor_x(105),
              y=box.text_anchor_y(105),
              width=box.text_anchor_width(105),
              height=box.text_anchor_height(105),
              bg_color="#ff99ff",
              z_level=-1)  # Set z-level to draw the box below the text

```

!!! note "Text anchor and `.code()`"

    Parsing text anchors is done through the same mechanism as text styles. You need to enable
    style parsing `parse_styles=True` in .code() for using text anchor in code.


## Box creation shortcuts

`box.line_box(line_idx)` is shortcut for

```python
box.box(x=box.line_x(line_idx),
        y=box.line_y(line_idy),
        width=box.line_width(line_idx),
        height=box.line_height(line_idx))`.
```

`box.text_anchor_box(anchor_id)` is shortcut for

```python
box.box(x=box.text_anchor_x(anchor_id),
        y=box.text_anchor_y(text_anchor_idy),
        width=box.text_anchor_width(anchor_id),
        height=text_anchor_height(anchor_id))`.
```

## Modifying value of a layout expression

When the layout is created, you cannot get the value of expression as the whole layout is not constructed yet;
however you can make a simple mathematical operations on expressions. Nelsie remebers them and applies them when the final value is computed.

```nelsie
@deck.slide()
def layout_expression_demo(slide):
    box1 = slide.box(width=300, height=100, bg_color="red")
    box2 = slide.box(width=150, height=200, bg_color="green")

    # Create a new box relative to the box1 x-position - 50 and box2 y-position + 100
    slide.box(x=box1.x() - 75, y=box2.y() + 100,
              width=50, height=50, bg_color="blue")
```

## Size scaling parameter

All of layout-expression creating methods take an optional `float` parameter, which sets the position or size
with respect to the fraction of the box size in the given dimension, e.g. `.x(0.5)` means the center of the box on the `X` axis. More precisely, it is defined as follows:

* `.x(v)` = `.x()` + v * `.width()`
* `.y(v)` = `.y()` + v * `.height()`
* `.width(v)` = v * `.width()`
* `.height(v)` = h * `.height()`

Example:

```nelsie
@deck.slide()
def layout_expression_demo(slide):
    box1 = slide.box(width=300, height=400, bg_color="red")

    slide.box(x=box1.x(0.5), y=box1.y(0.25),
              width=box1.width(0.5), height=box1.height(0.5),
              bg_color="green")
```

The same principle holds for other layout expressions:

* `.line_x(line_idx, v)` = `.line_x(line_idx)` + v * `.line_width(line_idx)`
* `.line_y(line_idx, v)` = `.line_y(line_idx)` + v * `.line_height(line_idx)`
* `.line_width(line_idx, v)` = v * `.line_width(line_idx)`
* `.line_height(line_idx, v)` = h * `.line_height(line_idx)`


* `.text_anchor_x(anchor_id, v)` = `.text_anchor_x(anchor_id)` + v * `.text_anchor_width(anchor_id)`
* `.text_anchor_y(anchor_id, v)` = `.text_anchor_y(anchor_id)` + v * `.text_anchor_height(anchor_id)`
* `.text_anchor_width(anchor_id, v)` = v * `.text_anchor_width(anchor_id)`
* `.text_anchor_height(anchor_id, v)` = h * `.text_anchor_height(anchor_id)`