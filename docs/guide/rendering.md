# Rendering

## Z-level

By default, boxes are rendered in depth-first order.
You can modify this by setting `z_level` to a box / Path / Rect / Oval.
This value is used in the painting order. Higher numbers are drawn later. If not set, the value is inherited from the
parent box. The default value of the slide root box is `0`. Z-level can be a negative integer.

```nelsie
@deck.slide()
def z_level_demo(slide):
    slide.box(x="10%", y="10%", width="50%", height="50%", bg_color="red")
    slide.box(x="20%", y="20%", z_level=2,
              width="50%", height="50%", bg_color="green")
    slide.box(x="30%", y="30%", width="50%", height="50%", bg_color="blue")

    stroke = Stroke(color="black", width=5)
    slide.add(Path(z_level=1, stroke=stroke)
        .move_to(Point(0, 0))
        .line_to(Point("100%", "100%")))
```

