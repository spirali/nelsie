# Layout

Nelsie uses a layout system that is based on the Flexbox and Grid system and adds some extra features.

The central element of the Nelsie layout system is the _Box_. A box is a rectangular area on a slide that has a position and size.
A box can contain other boxes or content (a text or an image).
Each Nelsie slide contains a hierarchical tree of boxes.
Boxes usually do not produce visual output directly, but they dictate how their children are arranged on a slide.

## Creating a box (via `.box()`)

A new box is created by calling the `.box()` method on an existing box or a slide. This will return a new box that will be a child of the element on which you call the box method.

This is a s minimal example where we create a box inside a slide:

```python
@deck.slide()
def first_box(slide):
    slide.box()
```

When we render this slide, we see an empty slide because the box itself produces no visual output.

To make them visible, in the next example we set a background color for the boxes. We also set them to a fixed size,
because by default box tries to be as small as possible and a box with no content takes up zero space.

Example where we create three boxes:

```nelsie
@deck.slide()
def three_boxes(slide):
    slide.box(width=600, height=200, bg_color="red")
    slide.box(width=600, height=200, bg_color="green")
    slide.box(width=600, height=200, bg_color="blue")
```

The full reference on `.box()` parameters is in the section [Box](box.md)

## Creating a box (standalone)

A box can be also created as a stand-alone method and attached into
the another box later. The following code is eqivalent to the example above:

```nelsie
from nelsie import Box

@deck.slide()
def three_boxes(slide):
    b1 = Box(width=600, height=200, bg_color="red")
    b2 = Box(width=600, height=200, bg_color="green")
    b3 = Box(width=600, height=200, bg_color="blue")

    slide.add(b1)
    slide.add(b2)
    slide.add(b3)
```


## Debugging layout

Another way how to visualize boxes is to set `debug_layout` to `True` in the slide configuration:

```nelsie
@deck.slide(debug_layout=True)
def debug_boxes(slide):
    slide.box(width=600, height=200)
    slide.box(width=600, height=200)
    slide.box(width=600, height=200)
```

For more configuration for debugging layout, see [Debugging layout](debug_layout.md).


## Box main axis

Boxes can have either be vertical or horizontal main axis:

* Vertical boxes place its child items vertically in a column. Their main axis is vertical and their cross axis is horizontal.

* Horizontal boxes place its child items horizontally in a row. Their main axis is horizontal and their cross axis is vertical.

By default, box is vertical. It can be changed by setting the parameter `row` to `True`:

```nelsie
@deck.slide()
def three_boxes(slide):

    my_box = slide.box(row=True)
    my_box.box(width=200, height=200, bg_color="red")
    my_box.box(width=200, height=200, bg_color="green")
    my_box.box(width=200, height=200, bg_color="blue")
```

!!! note "A box in a box"

    Box can contain other boxes. A box within the box can be created by calling the `.box()` method
    on the parent box. In this example, the slide's root box contains `my_box` and `my_box` contains three
    other boxes.

You can change the reverse order of child boxes by setting `reverse=True`.
It will arrange elements from left to right (or bottom to top if `row=True`).

```nelsie
@deck.slide()
def three_boxes(slide):

    my_box = slide.box(reverse=True)
    my_box.box(width=200, height=200, bg_color="red")
    my_box.box(width=200, height=200, bg_color="green")
    my_box.box(width=200, height=200, bg_color="blue")
```


## Box size

Each box has a width and a height. By default, the box tries to take up as little space as possible. It will
wraps its content tightly. If there is no content, the box has zero size.
This behaviour can be configured by setting the `width`, `height`, `flex-grow` and `flex-shrink` parameters.

### Width and Height

Weight/height parameters:

* `None` - (default) Automatic size. Minimum size around the content if `flex-grow` / `flex-shrink` is not set.
* `int` or `float` or `str` containing digits -- A fixed size given in pixels (example values: `20.5`, or `"1.5"`)
* `str` in format `"XX%"` where `XX` is an integer -- A relative size to the parent box, in percent (example: `"50%"`)
* `LayoutExpr` - A fixed size defined by a [layout expression](./layoutexpr.md).


### Flex grow

The `flex_grow` parameter takes a `float` value. The default is `0`. This attribute specifies how much of the remaining space of its parent box should be allocated to this box.

The remaining space is the size of the box minus the size of all its children. If multiple sibling boxes have positive `flex_grow' values, it is distributed according to the ratio defined by their values.

This property is equivalent to the CSS property `flex-grow'.

```nelsie
@deck.slide()
def flex_grow_demo(slide):
    slide.box(width=200, height=100, bg_color="red")
    slide.box(width=200, flex_grow=1, bg_color="green")
    slide.box(width=200, height=200, bg_color="blue")
```

## Padding & Margin

Padding (inner space) and margin (outer space) can be set via `p_left`, `p_right`, `p_top`, and `p_bottom` for setting padding and `m_left`, `m_right`, `m_top`, and `m_bottom` for setting a margin.

```nelsie
@deck.slide()
def flex_grow_demo(slide):
    my_box = slide.box(p_top=100, p_left=50, bg_color="red")
    my_box.box(width=200, height=200, bg_color="green")
```

There are also methods `padding` and `margin` that allows to set padding/margin later.

```python
slide.box(bg_color="red").padding(top=100, left=50)
```

This method also allows to set more padding/margin paramters at once:

following parameters for setting more padding/margin parameters at once:

* `all` that sets `left`, `right`, `top`, `bottom` to this value.
* `y` that sets `top` and `bottom`
* `x` that sets `left` and `right`

Example:

```python
slide.box().padding(all=5).margin(x=5)
```


## Arranging box children

Nelsie provides a [flexbox layout system](https://css-tricks.com/snippets/css/a-guide-to-flexbox/).
See [Flexbox froggy](https://flexboxfroggy.com/) for a nice tutorial.

Most of the layouts can be done via flexbox; however,
also supports grid layout, see [Grid layout](#grid-layout).

Nelsie supports from flexbox: `justify_content`, `align_items`, `align_self`, `align_items`, `align_self`, `justify_self`, `align_content`, `justify_content` and `gap_x` / `gap_y`.

The default configuration is `"center"` for configurations `justify_content` and `align_items`, i.e. items are put in the center on both axes.

### Example for `justify_content`

```nelsie
@deck.slide()
def justify_content_start(slide):
    b = slide.box(height="100%", justify_content="start")
    b.box(width=200, height=150, bg_color="red")
    b.box(width=200, height=150, bg_color="green")
    b.box(width=200, height=150, bg_color="blue")


@deck.slide()
def justify_content_end(slide):
    b = slide.box(height="100%", justify_content="end")
    b.box(width=200, height=150, bg_color="red")
    b.box(width=200, height=150, bg_color="green")
    b.box(width=200, height=150, bg_color="blue")


@deck.slide()
def justify_content_end(slide):
    b = slide.box(height="100%", justify_content="space-evenly")
    b.box(width=200, height=150, bg_color="red")
    b.box(width=200, height=150, bg_color="green")
    b.box(width=200, height=150, bg_color="blue")

```

## Fixed positioning of a box

You can set parameters `x` and `y` to set a fix position of the box independantly on the layout engine.

* `None` - (default) Coordianes are set by the layout engine.
* `int` or `float` or `str` containing digits -- A fixed position given relative to the parent box in pixels (example values: `20.5`, or `"1.5"`)
* `str` in format `"XX%"` where `XX` is an integer -- A fixed position relative to the parent box, in percent (example value: `"50%"` means that `x` (resp. `y`) is set to the 50% of width (resp. height) of the parent box)
* `LayoutExpr` - A fixed position defined by a [layout expression](./layoutexpr.md).

## Grid layout

Nelsie also supports [grid layout system](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_grid_layout/Basic_concepts_of_grid_layout); see [Grid garden](https://cssgridgarden.com/) for a nice tutorial.

```nelsie
from nelsie import GridOptions


@deck.slide()
def justify_content_start(slide):
    b = slide.box(
        width="100%", height="100%",
        grid=GridOptions(template_rows=("1 fr", "1fr"),
                         template_columns=("1 fr", "1fr")))
    b.box(grid=GridOptions(column=2, row=2), bg_color="orange").text("Grid 2-2")
```

Grid templates (`template_rows` and `template_columns`) may have values as follows:

* `200` or `"200"` - size of row/column in pixels
* `"50%"` - size of row/column in percents
* `"1 fr"` - size of row/column in fractions

Grid positions (`row` and `column`) may have values as follows:

* `2` - box at row/column `2`
* `(2, 5)` - box that spans from row/column 2 to row/column 5
* `(2, "span 3")` - box that spans from row/column 2 over 3 row/column.


### A rich table example

```nelsie

from nelsie import GridOptions as G

@deck.slide()
def grid_demo(slide):
    data = [
        ("Name", "Time", "Type"),
        ("Jane", 3.5, "A1"),
        ("John", 4.1, "B7"),
        ("Johanna", 12.0, "C1"),
        ("Elise", 12.5, "D4"),
        ("Max", 320.2, "E1")
    ]

    # Draw the table
    table = slide.box(
        width="70%",
        grid=G(template_columns=["2fr", "1fr", 130],
               template_rows=[50] + [40] * (len(data) - 1)),
        bg_color="#ddd",
    )
    header_style = TextStyle(weight=800)
    table.box(grid=G(column=(1, 4), row=1), bg_color="#fbc")
    for i in range(2, len(data) + 1, 2):
        table.box(grid=G(column=(1, 4), row=i), bg_color="#eee")
    column1 = table.box(grid=G(column=2, row=(1, len(data) + 1)))
    stroke = Stroke(color="#888", width=2)
    column1.add(Path(stroke=stroke).move_to(Point(0, 0)).line_to(Point(0, "100%")))
    column1.add(Path(stroke=stroke).move_to(Point("100%", 0)).line_to(Point("100%", "100%")))

    # Fill the table with data
    for i, row in enumerate(data, 1):
        s = header_style if i == 1 else None
        table.box(grid=G(column=1, row=i)).text(row[0], s)
        table.box(grid=G(column=2, row=i), row=True, justify_content="end", m_right=30).text(str(row[1]), s)
        table.box(grid=G(column=3, row=i), row=True, justify_content="start", m_left=30).text(row[2], s)
```

## Method `.overlay()`

There is a `.overlay()` method that is a shortcut for `.box(x=0, y=0, width="100%", height="100%")`;
it creates a box that spans over the whole parent box.
