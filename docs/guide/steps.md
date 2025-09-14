# Steps

Presentations often contain slides that are revealed gradually in several steps.Nelsie allows you to easily create
multiple steps per slide and selectively show or configure individual elements in each step.

Each slide can contain one or more steps, and each step will produce one page in the resulting presentation. What is
shown in each step is configured by the `show` and `active` parameters of the box and the `StepVal` instance.

```nelsie
@deck.slide()
def show_demo(slide):

    slide.set_style("default", TextStyle(size=80))  # Just change size of font

    # This slide generates 3 output pages

    slide.box().text("One")             # Show always
    slide.box(show="2+").text("Two")    # Show from step 2
    slide.box(show="3+").text("Three")  # Show from step 3
```

In the following examples we first introduce steps as natural numbers, but later we will show they may have hierarchical
structure, similars to a sections and subsections to a book. For example 1.1, 1.2, etc.

By default, slide is unfolded to all steps that occur in `show`, `active`, or `InSteps` and step `1`.
Step `1` is added automatically even it is not named anywhere. You can disable this behavior by setting parameter
`step_1=False` when creating a new slide. What steps are rendered can be modified various means.
Step `0` is special; It is a valid step but it is never shown even it is named.

## Box `show` parameter

Parameter `show` in `.box()` defines the steps in which the box (its contents and children) is displayed. It only
affects the drawing itself, but not the layout. The layout is always calculated, i.e. the space is reserved for the box
and its children, even in the steps where it is not painted.

### Example 1 (showing new content and hinding old content)

```nelsie
@deck.slide()
def show_demo(slide):

    slide.set_style("default", TextStyle(size=80))  # Just change size of font

    # This slide generates 3 output pages

    slide.box(show=1).text("One")    # Show only in step 1
    slide.box(show=2).text("Two")    # Show only in step 2
    slide.box(show=3).text("Three")  # Show only in step 3
```

### Example 2 (rendering only named steps)

The following code creates the same resulting pages are the previous case,
despite it uses different steps. The reason is that Nelsie only renders steps that are named.

!!! note "Debug steps"

    We are also enabling `debug_steps` in this example. It attaches a black block under the slide with
    the current step.

```nelsie
@deck.slide(debug_steps=True)
def show_demo(slide):

    slide.set_style("default", TextStyle(size=80))  # Just change size of font

    # This slide generates 3 output pages

    slide.box(show=1).text("One")     # Show only in step 1
    slide.box(show=20).text("Two")    # Show only in step 20
    slide.box(show=30).text("Three")  # Show only in step 30
```

### Example 3 (string definitions)

The `show` argument may also define more complex step definitions as strings.
Note, that range `X-Y` covers all steps in the range, however it forces to create only
steps `X` and `Y`. The following example creates 4 pages, that are steps: 1,2,4,10

```nelsie
@deck.slide(debug_steps=True)
def show_demo(slide):

    slide.set_style("default", TextStyle(size=80))  # Just change size of font

    slide.box(show="2+").text("One")     # Show from step two
    slide.box(show="2-10").text("Two")   # Show in steps between 2-10, both ends included
    slide.box(show="1,4").text("Three")  # Show in steps 1 and 4
```

### Values for `show`

Parameter `show` takes the following types:

* `bool` - the box is always shown (`True`) or hidden (`False`).
* `int` - the box is shown only in the given step
* `str` - a string may have the following format:
    * `"<step>"` - the box is shown only in the given step
    * `"<step>+"` - the box is shown in the given step and all following steps
    * `"<step>-<step>"` - the box is shown in the steps in the given range.
    * Comma separated list of the expression above. Then the box is shown in the union of steps defined by expressions.
      Example: `"1, 5, 20-30, 35+"`.

### Silent steps

A step may be silenced by syntax `X?`:

```nelsie
@deck.slide(debug_steps=True)
def show_demo(slide):

    slide.set_style("default", TextStyle(size=80))  # Just change size of font

    # This slide generates 2 output pages (step 1 and 3)

    slide.box(show="1-3").text("One")     # Show from step two to three
    slide.box(show="2?+").text("Two")     # Show in steps 2 and more, but does not force
                                          # an existence of step 2
```

## Box `active` parameter

Box `active` parameter is similar to `show`. It takes parameters as `show`, but
in steps when the box is not active, it is also removed from the layout, i.e. no space is reserved for the box.

```nelsie
@deck.slide()
def show_demo(slide):

    slide.set_style("default", TextStyle(size=80))  # Just change size of font

    # This slide generates 3 output pages

    slide.box(active=1).text("One")
    slide.box(active=2).text("Two")
    slide.box(active=3).text("Three")
```

## `StepVal` class

`StepVal` defines different values for step of slides.
You can pass `StepVal` instance to almost all parameters of `.box()`.
To find parameters where `StepVal` can be used,
see [API documentation](https://spirali.github.io/nelsie/api_doc/nelsie.html#BoxBuilder.box).
If type definition contains `StepVal` then it can be used for this parameter.

### Simple example

```nelsie

from nelsie import StepVal

@deck.slide()
def stepval_demo(slide):

    # Define different colors in tree steps.
    # "red" is default, from step 2 we set "green", from step 3 we set "blue"
    bg_colors = StepVal("red").at(2, "green").at(3, "blue")

    slide.box(width=300, height=300, bg_color=bg_colors)
```

### Semaphore example

More instances of `StepVal` can be used at once:

```nelsie
@deck.slide()
def semaphore(slide):
    slide.text("Semaphore example", m_bottom=40)

    semaphore = slide.box(width=200, height=600, bg_color="gray")
    semaphore.box(
        # Configure "y" coordinate in each step
        y=StepVal(20).at(2, 220).at(3, 420),

        width=160,
        height=160,

        # Configure "bg_color" in each step
        bg_color=StepVal("red").at(2, "orange").at(3, "green"),
    )
```

### Undefined step values

When `StepVal` is used, you do not need to define all step values.
If no value is defined for a given step, then a value for the next smaller step is used. If no such step is defined,
then the default for the parameter is used.

```nelsie
@deck.slide()
def stepval_demo(slide):

    slide.set_style("default", TextStyle(size=60))

    # Define different colors in steps: 2 and 4
    bg_colors = StepVal().at(2, "red").at(4, "green")

    box = slide.box(width=300, height=300, bg_color=bg_colors)
    for i in range(1, 6):
        box.text(f"Step {i}", active=i)
```

## Configuring steps

The visible steps is automatically determined from the values in `show`, `active`, or the use of `StepVal`.
You can override this by calling `insert_step` and `ignore_steps` on a slide:

```nelsie
@deck.slide()
def stepval_demo(slide):

    # Define different colors in steps: 1, 2, and 3
    bg_colors = StepVal("red").at(2, "green").at(3, "blue").at(5, "magenta")

    slide.box(width=300, height=300, bg_color=bg_colors)

    slide.insert_step(4)
    slide.ignore_steps("2-3")
    # This slide will have only 3 steps (steps 1, 4, 5)
```

The `.ignore_steps` takes the same format as `show`/`active`.

## Hierarchical steps

Steps does not have to be only natural numbers, but they may have hierarchal structure, like in a sections and
subsections of a book (example: 1.1, 1.2, 5.1.1). Formally a step is a non-empty sequence of natural numbers. Step are
lexicographically ordered.

Hierarchical steps are defined as a sequence of integers. Step defined as a just `int` is
equivalent to a sequence with a single element.
A hierarchical step is defined as a string where `.` is used as a delimiter, e.g. `"2.3.1"`.

```nelsie
@deck.slide(debug_steps=True)
def step_demo1(slide):

    slide.set_style("default", TextStyle(size=80))  # Just change size of font

    # This slide generates 3 output pages

    slide.box(show=1).text("One")
    slide.box(show="1.1").text("Two")
    slide.box(show="1.2").text("Three")
    slide.box(show=2).text("Four")
```

Note that if you define that something should be shown (or active) in a step then it is shown (or active) in all of its
substeps.
If you want to exclude substeps you have to use `!` character before the step definition:

```nelsie
@deck.slide(debug_steps=True)
def step_demo2(slide):
    slide.set_style("default", TextStyle(size=80))  # Just change size of font

    # This slide generates 3 output pages

    slide.box(show="!1").text("One")
    slide.box(show="1.1").text("Two")
    slide.box(show="1.2").text("Three")
    slide.box(show=2).text("Four")
```

## Step counters

Step counters can be used when you want to avoid naming current numbers.

```nelsie
from nelsie import StepCounter


@deck.slide()
def counter_demo(slide):
    c = StepCounter()
    slide.text("Line 1")
    slide.text("Line 2", show=c.next_p())
    slide.text("Line 3", show=c.next_p())
    slide.text("Line 4", show=c.next_p())
```

* `.last()` returns the current value of the counter
* `.last_p()` (as "last plus") return a string `"{self.last()}+"`, i.e. the current value of the counter plus all steps
  above.
* `.next()` increments the counter and returns the new value
* `.next_p()` increments the counter and returns a string `"{self.next()}+"`, i.e. the new value of the counter plus all
  steps above.