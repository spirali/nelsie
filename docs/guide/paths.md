# Paths

A path is a sequence of straight line segments and/or Bezier curves. A path may be filled and/or stroked. A path may
have attached arrow to beginning and/or end.

Paths are drawn in Nelsie by constructing a `Path` instance and calling `.add()` method
on a box. Path can be stroked or filled.

```nelsie
from nelsie import Path, Stroke, Point


@deck.slide()
def path_demo(slide):
    stroke = Stroke(color="orange", width=40)
    path = Path(fill_color="blue", stroke=stroke) \
               .move_to(Point(204, 300)) \
               .line_to(Point(819, 300)) \
               .line_to(Point(512, 600))
    slide.add(path)
```

## `Path` methods

`Path` constructor create an empty instance of a path.
You can create line segments or Bezeier curves by calling the following methods:

* `.move_to(point)` -- move the cursor to a given position without visible effect
* `.move_by(x, y)` -- move the cursor by a given `(x, y)` offset, relative to the last position set by any `*_to` method
* `.line_to(point)` -- create a line segment
* `.line_by(x, y)` -- create a line segment that ends at the given `(x, y)` offset, relative to the last position set by
  any `*_to` method
* `.quad_to(p1, p)` -- create a quadratic bezier curve
* `.cubic_to(p1, p2, p)` -- create a cubic bezier curve
* `.close()` -- create a line segment to the initial point of the path

```nelsie
@deck.slide()
def path_demo(slide):
    path = Path(fill_color="red") \
        .move_to(Point(200, 600)) \
        .quad_to(Point(500, 200), Point(800, 600)) \
        .close()
    slide.add(path)
```

## Path coordinates

Path coordinates can be given as:

* `float` or a string with digits, (e.g. `100`, `"100"`) - a position in pixels relative to a box where a path is drawn
* string in format "XX%" (e.g. `"50%"`) - a position relative to the box where the path is drawn with respect to its
  size.
* `LayoutExpr` - a position defined as a layout expression (see [Layout expressions](layoutexpr.md))

```nelsie
@deck.slide()
def path_demo(slide):
    path = Path(fill_color="red") \
        .move_to(Point("50%", "50%")) \
        .line_to(Point("100%", "100%")) \
        .line_to(Point("50%", "100%"))
    slide.add(path)
```

```nelsie
@deck.slide()
def path_demo(slide):
    box = slide.text("Hello world", TextStyle(size=80), bg_color="orange")

    # box.p() returns Point that is relative to the box
    path = Path(fill_color="red") \
        .move_to(box.p(0, 1.0)) \
        .line_to(Point("100%", "100%")) \
        .line_to(Point("50%", "100%"))
    slide.add(path)
```

## `Rect` and `Oval` classes

Rect and Oval classes are convenience classes that create a path with a rectangle or oval shape.

```nelsie
from nelsie import Oval, Rect


@deck.slide()
def path_demo(slide):
    shape = Rect(Point(400, 500), Point(600, 700), fill_color="orange")
    slide.add(shape)
    stroke = Stroke(color="orange", width=10)
    shape = Oval(Point(200, 200), Point(400, 400), fill_color="green", stroke=stroke)
    slide.add(shape)
```

## `Stroke` class

`Stroke` class defines how a path is stroked; you can configure a color, width, and line dash.

```nelsie
from nelsie import Stroke


@deck.slide()
def stroke_demo(slide):
    box = slide.box(width=700, height=140, m_bottom=60)
    box.add(Path(stroke=Stroke(color="red", width=10)).move_to(Point(0, 0)) \
        .line_to(Point(700, 0)))
    box.add(Path(stroke=Stroke(color="green", width=20, dash_array=[10, 20], dash_offset=15)) \
         .move_to(Point(0, "50%")) \
         .line_to(Point(700, "50%")))
    stroke = Stroke(color="blue", width=30, dash_array=[30, 10, 5, 10])
    box.add(Path(stroke=stroke).move_to(Point(0, "100%")).line_to(Point(700, "100%")))
```

## Arrows

You can attach an arrow to the beginning and/or end of a path.
An arrow is created as an instance of `Arrow` and can be passed to the constructor of a
can be passed to the constructor of `Path` in the parameters `arrow_start` and `arrow_end`.
A color, size, angle, line width and position of the inner point can be configured.

```nelsie
from nelsie import Arrow

@deck.slide()
def arrow_demo(slide):
    box = slide.box(width=700, height=220)
    arrow1 = Arrow(size=80)
    stroke = Stroke(color="black", width=10)
    box.add(
        Path(stroke=stroke, arrow_start=arrow1, arrow_end=arrow1)
            .move_to(Point(0, 0))
            .line_to(Point(700, 0)),
    )
```

### Color of arrows

If parameter `color` is not defined, then arrow will have the same color as the path.

```nelsie
@deck.slide()
def arrow_demo(slide):
    box = slide.box(width=700, height=220)
    stroke = Stroke(color="green", width=10)
    box.add(Path(stroke=stroke, arrow_end=Arrow(size=80)) \
        .move_to(Point(0, 0)) \
        .line_to(Point(700, 0)))
    box.add(Path(stroke=stroke, arrow_end=Arrow(size=80, color="red")) \
        .move_to(Point(0, 120)) \
        .line_to(Point(700, 120)))
```

### Size of arrows

```nelsie
@deck.slide()
def arrow_demo(slide):
    box = slide.box(width=700, height=220)
    stroke = Stroke(color="black", width=10)
    path = Path(stroke=stroke, arrow_end=Arrow(size=30)) \
        .move_to(Point(0, 0)) \
        .line_to(Point(700, 0))
    box.add(path)
    path = Path(stroke=stroke, arrow_end=Arrow(size=80)) \
        .move_to(Point(0, 150)) \
        .line_to(Point(700, 150))
    box.add(path)
    path = Path(stroke=stroke, arrow_end=Arrow(size=120)) \
        .move_to(Point(0, 300)) \
        .line_to(Point(700, 300))
    box.add(path)
```

### Angle of arrows

```nelsie
from nelsie import Arrow


@deck.slide()
def arrow_demo(slide):
    box = slide.box(width=700, height=220)
    stroke = Stroke(color="black", width=10)
    box.add(Path(stroke=stroke, arrow_end=Arrow(size=80, angle=60))
                .move_to(Point(0, 0))
                .line_to(Point(700, 0)))
    box.add(Path(stroke=stroke, arrow_end=Arrow(size=80, angle=45))
                .move_to(Point(0, 150))
                .line_to(Point(700, 150)))
    box.add(Path(stroke=stroke, arrow_end=Arrow(size=80, angle=20))
                .move_to(Point(0, 300))
                .line_to(Point(700, 300)))
```

### Stroked arrows

If `stroked_width` is not `None` then the arrow is not filled but stroked.

```nelsie
@deck.slide()
def arrow_demo(slide):
    box = slide.box(width=700, height=220)
    stroke = Stroke(color="black", width=10)
    box.add(Path(stroke=stroke, arrow_end=Arrow(size=80))
            .move_to(Point(0, 0))
            .line_to(Point(700, 0)))
    box.add(Path(stroke=stroke, arrow_end=Arrow(size=80, stroke_width=10))
            .move_to(Point(0, 150))
            .line_to(Point(700, 150)))
    box.add(Path(stroke=stroke, arrow_end=Arrow(size=80, stroke_width=30))
            .move_to(Point(0, 300))
            .line_to(Point(700, 300)))
```

### Inner point

```nelsie
@deck.slide()
def arrow_demo(slide):
    box = slide.box(width=700, height=220)
    stroke = Stroke(color="black", width=10)
    box.add(Path(stroke=stroke, arrow_end=Arrow(size=80, inner_point=0.5))
            .move_to(Point(0, 0))
            .line_to(Point(700, 0)))
    box.add(Path(stroke=stroke, arrow_end=Arrow(size=80, inner_point=1.0))
            .move_to(Point(0, 150))
            .line_to(Point(700, 150)))
    box.add(Path(stroke=stroke, arrow_end=Arrow(size=80, inner_point=2.5))
            .move_to(Point(0, 300))
            .line_to(Point(700, 300)))
```