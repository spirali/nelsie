# Paths

A path is a sequence of straight line segments and/or Bezier curves. A path may be filled and/or stroked. A path may have attached arrow to beginning and/or end.

Paths are drawn in Nelsie by constructing a `Path` instance and calling `.draw()` method
on a box. Path can be stroked

```nelsie
from nelsie import Path, Stroke

@deck.slide()
def path_demo(slide):
    stroke = Stroke(color="orange", width=40)
    path = Path(fill_color="blue", stroke=stroke) \
            .move_to(204, 300) \
            .line_to(819, 300) \
            .line_to(512, 600)
    slide.draw(path)
```

## `Path` methods

`Path` constructor create an empty instance of a path.
You can create line segments or Bezeier curves by calling the following methods:

* `.move_to(x, y)` -- move the cursor to a given position without visible effect
* `.line_to(x, y)` -- create a line segment
* `.quad_to(x1, y1, x, y)` -- create a quadratic bezier curve
* `.cubic_to(x1, y1, x2, y2, x, y)` -- create a cubic bezier curve
* `.close()` -- create a line segment to the initial point of the path


```nelsie
@deck.slide()
def path_demo(slide):
    path = Path(fill_color="red") \
             .move_to(200, 600) \
             .quad_to(500, 200, 800, 600) \
             .close()
    slide.draw(path)
```

## Path coordinates

Path coordinates can be given as:

* `float` or a string with digits, (e.g. `100`, `"100"`) - a position in pixels relative to a box where a path is drawn
* string in format "XX%" (e.g. `"50%"`) - a position relative to the box where the path is drawn with respect to its size.
* `LayoutExpr` - a position defined as a layout expression (see [Layout expressions](layoutexpr.md))

```nelsie
@deck.slide()
def path_demo(slide):
    path = Path(fill_color="red") \
             .move_to("50%", "50%") \
             .line_to("100%", "100%") \
             .line_to("50%", "100%")
    slide.draw(path)
```

```nelsie
@deck.slide()
def path_demo(slide):
    box = slide.text("Hello world", TextStyle(size=80), bg_color="orange")
    path = Path(fill_color="red") \
             .move_to(box.x(), box.y(1.0)) \
             .line_to("100%", "100%") \
             .line_to("50%", "100%")
    slide.draw(path)
```

## `Path.oval` method

You can create a Path representing an oval (or a circle) by calling `Path.oval(x1, y1, x2, y2)`

```nelsie
from nelsie import Path

@deck.slide()
def path_demo(slide):
    path = Path.oval(200, 200, 400, 400, fill_color="green")
    slide.draw(path)
```

## Drawing more paths

A path is drawn by calling `.draw()` method on a box. You can draw more paths at once by providing a list of paths.

```python
box.draw([
    Path(...), 
    Path(...),
    Path(...)
])
```

## `Stroke` class

`Stroke` class defines how a path is stroked; you can configure a color, width, and line dash.

```nelsie
from nelsie import Stroke

@deck.slide()
def stroke_demo(slide):
    box = slide.box(width=700, height=140, m_bottom=60)
    box.draw(
        [
            Path(stroke=Stroke(color="red", width=10)).move_to(0, 0).line_to(700, 0),
            Path(stroke=Stroke(color="green", width=20, dash_array=[10, 20], dash_offset=15))
            .move_to(0, "50%")
            .line_to(700, "50%"),
            Path(stroke=Stroke(color="blue", width=30, dash_array=[30, 10, 5, 10])).move_to(0, "100%").line_to(700, "100%"),
        ]
    )
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
    box.draw(
        Path(stroke=stroke, arrow_start=arrow1, arrow_end=arrow1)
            .move_to(0, 0)
            .line_to(700, 0),
    )
```


### Color of arrows

If parameter `color` is not defined, then arrow will have the same color as the path.

```nelsie
@deck.slide()
def arrow_demo(slide):
    box = slide.box(width=700, height=220)
    stroke = Stroke(color="green", width=10)
    box.draw(
        [
            Path(stroke=stroke, arrow_end=Arrow(size=80))
                .move_to(0, 0)
                .line_to(700, 0),
            Path(stroke=stroke, arrow_end=Arrow(size=80, color="red"))
                .move_to(0, 120)
                .line_to(700, 120),
        ]
    )
```


### Size of arrows

```nelsie
@deck.slide()
def arrow_demo(slide):
    box = slide.box(width=700, height=220)
    stroke = Stroke(color="black", width=10)
    box.draw(
        [
            Path(stroke=stroke, arrow_end=Arrow(size=30))
                .move_to(0, 0)
                .line_to(700, 0),
            Path(stroke=stroke, arrow_end=Arrow(size=80))
                .move_to(0, 150)
                .line_to(700, 150),
            Path(stroke=stroke, arrow_end=Arrow(size=120))
                .move_to(0, 300)
                .line_to(700, 300),
        ]
    )
```

### Angle of arrows

```nelsie
from nelsie import Arrow

@deck.slide()
def arrow_demo(slide):
    box = slide.box(width=700, height=220)
    stroke = Stroke(color="black", width=10)
    box.draw(
        [
            Path(stroke=stroke, arrow_end=Arrow(size=80, angle=60))
                .move_to(0, 0)
                .line_to(700, 0),
            Path(stroke=stroke, arrow_end=Arrow(size=80, angle=45))
                .move_to(0, 150)
                .line_to(700, 150),
            Path(stroke=stroke, arrow_end=Arrow(size=80, angle=20))
                .move_to(0, 300)
                .line_to(700, 300),
        ]
    )
```


### Stroked arrows

If `stroked_width` is not `None` then the arrow is not filled but stroked.

```nelsie
@deck.slide()
def arrow_demo(slide):
    box = slide.box(width=700, height=220)
    stroke = Stroke(color="black", width=10)
    box.draw(
        [
            Path(stroke=stroke, arrow_end=Arrow(size=80))
                .move_to(0, 0)
                .line_to(700, 0),
            Path(stroke=stroke, arrow_end=Arrow(size=80, stroke_width=10))
                .move_to(0, 150)
                .line_to(700, 150),
            Path(stroke=stroke, arrow_end=Arrow(size=80, stroke_width=30))
                .move_to(0, 300)
                .line_to(700, 300),
        ]
    )
```


### Inner point

```nelsie
@deck.slide()
def arrow_demo(slide):
    box = slide.box(width=700, height=220)
    stroke = Stroke(color="black", width=10)
    box.draw(
        [
            Path(stroke=stroke, arrow_end=Arrow(size=80, inner_point=0.5))
                .move_to(0, 0)
                .line_to(700, 0),
            Path(stroke=stroke, arrow_end=Arrow(size=80, inner_point=1.0))
                .move_to(0, 150)
                .line_to(700, 150),
            Path(stroke=stroke, arrow_end=Arrow(size=80, inner_point=2.5))
                .move_to(0, 300)
                .line_to(700, 300),
        ]
    )
```


## `InSteps` and paths

Parameters of `Path` and `Arrow` do not take `InSteps` values; however `.draw()` method of the box accepts
`InSteps` values:

```nelsie
@deck.slide()
def draw_insteps_demo(slide):
    box = slide.box(width=700, height=220)
    stroke = Stroke(color="black", width=10)
    
    path1 = Path(stroke=stroke).move_to(0, 0).line_to(700, 0)
    path2 = Path(stroke=stroke).move_to(0, 100).line_to(700, 100)
    path3 = Path(stroke=stroke).move_to(0, 200).line_to(700, 200)
                  
    box.draw(InSteps({1 : path1, 2: [path2, path3]}))
```


