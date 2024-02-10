# Layout

Nelsie uses a layout system that is based on the Flexbox system and adds some extra features.

The central element of the Nelsie layout system is the _Box_.
Each Nelsie slide contains a layout hierarchy tree of boxes. Boxes do not directly produce visual output, but they dictate how their children are laid out on a slide.


# Creating a box

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

# Debugging layout

Another way how to visualize boxes is to set `debug_layout` to `True` in the slide configuration: 

```nelsie
@deck.slide(debug_layout=True)
def debug_boxes(slide):
    slide.box(width=600, height=200)
    slide.box(width=600, height=200)
    slide.box(width=600, height=200)
```

For more configuration for debugging layout, see [Debugging layout](../reference/debug_layout.md).


# Box main axis

Boxes can have either be vertical or horizontal main axis:

* Vertical boxes place its child items vertically in a column. Their main axis is vertical and their cross axis is horizontal.

* Horizontal boxes place its child items horizontally in a row. Their main axis is horizontal and their cross axis is vertical.

By default, box is vertical. It can be changed by setting the parameter `row`:

```nelsie
@deck.slide()
def three_boxes(slide):

    my_box = slide.box(row=True)
    my_box.box(width=200, height=200, bg_color="red")
    my_box.box(width=200, height=200, bg_color="green")
    my_box.box(width=200, height=200, bg_color="blue")
```
