# Steps

Presentations often contain slides that are revealed gradually in several steps.Nelsie allows you to easily create multiple steps per slide and selectively show or configure individual elements in each step.

Each slide can contain one or more steps, and each step will produce one page in the resulting presentation. What is shown in each step is configured by the `show` and `active` parameters of the box and the `InSteps` instance.
Slide steps are counted from 1. By default, the number of steps in a slide is determined by the maximum number of steps that occur in `show`, `active`, or `InSteps`; however, there is always at least one step, step `1`.


## Box `show` parameter

Parameter `show` in `.box()` defines the steps in which the box (its contents and children) is displayed. It only affects the drawing itself, but not the layout. The layout is always calculated, i.e. the space is reserved for the box and its children, even in the steps where it is not painted.

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

### Example 2 (showing new content while keeping the old)

```nelsie
@deck.slide()
def show_demo(slide):

    slide.set_style("default", TextStyle(size=80))  # Just change size of font

    # This slide generates 3 output pages

    slide.box().text("One")             # Show always
    slide.box(show="2+").text("Two")    # Show from step 2
    slide.box(show="3+").text("Three")  # Show from step 3
```

### Values for `show`

Parameter `show` takes the following types:

* `bool` - the box is always shown (`True`) or hidden (`False`).
* `int` - the box is shown only in the given step
* `str` - a string may have the following format:
    * `"<number>"` - the box is shown only in the given step
    * `"<number>+"` - the box is shown in the given step and all following steps
    * `"<number>-<number>"` - the box is shown in the steps in the given range.
    * Comma separated list of the expression above. Then the box is shown in the union of steps defined by expressions. Example: `"1, 5, 20-30, 35+"`.



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


## `InSteps` class

`InSteps` defines different values for step of slides.
You can pass `InSteps` instance to almost all parameters of `.box()`.
To find parameters where `InSteps` can be used, see [API documentation](https://spirali.github.io/nelsie/api_doc/nelsie.html#BoxBuilder.box).
If type definition contains `InSteps` then it can be used for this parameter.


### Simple example

```nelsie
from nelsie import InSteps

@deck.slide()
def inslides_demo(slide):

    # Define different colors in steps: 1, 2, and 3
    bg_colors = InSteps({1: "red", 2: "green", 3: "blue"})

    slide.box(width=300, height=300, bg_color=bg_colors)
```

### Semaphore example

More instances of `InSteps` can be used at once:

```nelsie
@deck.slide()
def semaphore(slide):
    slide.text("Semaphore example", m_bottom=40)

    semaphore = slide.box(width=200, height=600, bg_color="gray")
    semaphore.box(
        # Configure "y" coordinate in each step
        y=InSteps({1: 20, 2: 220, 3: 420}),

        width=160,
        height=160,

        # Configure "bg_color" in each step
        bg_color=InSteps({1: "red", 2: "orange", 3: "green"}),
    )
```

### Undefined step values

When `InSteps` is used, you do not need to define all step values.
If no value is defined for a given step, then a value for the next smaller step is used. If no such step is defined, then the default for the parameter is used.

```nelsie
from nelsie import InSteps

@deck.slide()
def inslides_demo(slide):

    slide.set_style("default", TextStyle(size=60))

    # Define different colors in steps: 2 and 4
    bg_colors = InSteps({2: "red", 4: "green"})

    box = slide.box(width=300, height=300, bg_color=bg_colors)
    for i in range(1, 6):
        box.text(f"Step {i}", active=i)
```


### `InSteps` initialized by a list

Instead of directory, `InSteps` can be also initialized by a list of values.
It then defines values for first n steps where n is a length of the list.

```python
InSteps(["red", "green", "blue"])

# Is equivalent to:

InSteps({1: "red", 2: "green", 3: "blue"})
```

## Configuring the number of steps

The number of steps is automatically determined from the values in `show`, `active`, or the use of `InSteps`. You can override this by calling `set_n_steps` on a slide:

```nelsie
from nelsie import InSteps

@deck.slide()
def inslides_demo(slide):

    # Define different colors in steps: 1, 2, and 3
    bg_colors = InSteps({1: "red", 2: "green", 3: "blue"})

    slide.box(width=300, height=300, bg_color=bg_colors)

    # This slide will have only 2 steps, even we have defined color for step 3
    slide.set_n_steps(2)
```

The `.set_n_steps()` method should be called after all step values have been configured, because any subsequent call of step configuration may again increase the final number of steps.

You get the information of the current number of steps for a slide by calling:

```python
slide.get_n_steps()
```

## `last`, `last+`, `next`, `next+` keywords

Box `show` and `active` takes also the following keywords:

* "last" is equvalent to `slide.get_n_steps()`
* "last+" is equvalent to `f"{slide.get_n_steps()}+"`
* "next" is equvalent to `slide.get_n_steps() + 1`
* "next+" is equvalent to `f"{slide.get_n_steps() + 1}+"`

```nelsie
@deck.slide()
def keywords_demo(slide):
    slide.text("Line 1")
    slide.text("Line 2", show="next+")
    slide.text("Line 3", show="next+")
    slide.text("Line 4", show="last")
```


## Step replacing

Step replacing is change the "current step" value for a subtree of box hierarchy.
It is usefull when you already have a code (or an image with steps) and you want show
just a selection of steps or reorder steps without change of the source.


```nelsie
@deck.slide()
def show_demo(slide):

    slide.set_style("default", TextStyle(size=80))  # Just change size of font

    # For "main" box and its children, 1st step will be overriden to 3rd step
    main = slide.box(replace_steps={1: 3})

    main.box(show=1).text("One")    # Show only in step 1
    main.box(show=2).text("Two")    # Show only in step 2
    main.box(show=3).text("Three")  # Show only in step 3
```