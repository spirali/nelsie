# Debugging layout

This page describes how you an debug a layout boxes by visualizing them.


## Enabling `debug_layout` for whole slide

To visualize layout boxes you can set `debug_layout` to `True` in the slide configuration: 

```nelsie
@deck.slide(debug_layout=True)
def example_slide(slide):
    slide.box(width=600, height=200)
    slide.box(width=600, height=200)
    slide.box(width=600, height=200)
```

If the default color for displaying debug frames of boxes conflicts with the colors of your slide's content or background, you can define your own color for drawing.


```nelsie
@deck.slide(debug_layout="green")
def example_slide(slide):
    slide.box(width=600, height=200)
    slide.box(width=600, height=200)
    slide.box(width=600, height=200)
```

## Enabling `debug_layout` for a box

You can also change the color only for a specific box by setting the `debug_layout` parameter of the box.
You can also disable the highlighting of a specific box frame by setting `debug_layout` to `False`.

```nelsie
@deck.slide(debug_layout=True)
def example_slide(slide):
    slide.box(width=600, height=200)

    # Set green frame for this box
    slide.box(width=600, height=200, debug_layout="green")

    # Disable layout debugging for this box
    slide.box(width=600, height=200, debug_layout=False)
```

Or you can just enable the debug frames only for a specific box:

```nelsie
@deck.slide()
def example_slide(slide):
    slide.box(width=600, height=200)

     # Show the debug frame for this box
    slide.box(width=600, height=200, debug_layout="green")
    
    slide.box(width=600, height=200)
```


# Box naming

You can name a box by setting the `name` parameter. The name has no effect on a layout, but it is displayed when debug frames are enabled. It may help you to find the box when there are too many boxes.

Note: The slide previews in the documentation are too small to see the name. If you render the example in PDF, you will see it.

```nelsie
@deck.slide(debug_layout=True)
def box_with_names(slide):
    slide.box(width=600, height=200, name=">>> My-box <<<")
```