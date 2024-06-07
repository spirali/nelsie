# Steps

Presentations often contain slides that are revealed gradually in several steps.Nelsie allows you to easily create multiple steps per slide and selectively show or configure individual elements in each step.

Each slide can contain one or more steps, and each step will produce one page in the resulting presentation. What is shown in each step is configured by the `show` and `active` parameters of the box and the `InSteps` instance.


```nelsie
@deck.slide()
def show_demo(slide):

    slide.set_style("default", TextStyle(size=80))  # Just change size of font

    # This slide generates 3 output pages

    slide.box().text("One")             # Show always
    slide.box(show="2+").text("Two")    # Show from step 2
    slide.box(show="3+").text("Three")  # Show from step 3
```

In the following examples we first introduce steps as natural numbers, but later we will show they may have hierarchical structure, similars to a sections and subsections to a book. For example 1.1, 1.2, etc.

By default, slide is unfolded to all steps that occur in `show`, `active`, or `InSteps` and step `1`. 
Step `1` is added automatically even it is not named anywhere. You can disable this behavior by setting parameter `step_1=False` when creating a new slide. What steps are rendered can be modified various means.
Step `0` is special; It is a valid step but it is never shown even it is named.

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
    * Comma separated list of the expression above. Then the box is shown in the union of steps defined by expressions. Example: `"1, 5, 20-30, 35+"`.


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

## Configuring steps

The visible steps is automatically determined from the values in `show`, `active`, or the use of `InSteps`. 
You can override this by calling `insert_step` and `remove_step` on a slide:

```nelsie
from nelsie import InSteps

@deck.slide()
def inslides_demo(slide):

    # Define different colors in steps: 1, 2, and 3
    bg_colors = InSteps({1: "red", 2: "green", 3: "blue"})

    slide.box(width=300, height=300, bg_color=bg_colors)

    # This slide will have only 3 steps, 
    slide.remove_step(2)
    slide.insert_step(4)
```

The `.remove_step` method should be called after all step values have been configured, because any subsequent call of step configuration may again increase the final number of steps. 
There is also `.remove_steps_below` and `.remove_steps_above` to remove all steps below (resp. above) a given step.

You get the information of visible steps for a slide by calling `get_steps`. The method returns sorted steps:

```python
slide.get_steps()
```

## Hierarchical steps

Steps does not have to be only natural numbers, but they may have hierarchal structure, like in a sections and subsections of a book (example: 1.1, 1.2, 5.1.1). Formally a step is a non-empty sequence of natural numbers. Step are lexicographically ordered.

Hierarchical steps are defined as a tuple of `int`s if defined as Python object. Step defined as a just `int` is equivalent to a tuple with a single element. 
If a step is defined in a string form then `.` is used as a delimiter. Example: `(2, 3)` is equivalent to `"2.3"`.


```nelsie
@deck.slide(debug_steps=True)
def step_demo1(slide):

    slide.set_style("default", TextStyle(size=80))  # Just change size of font

    # This slide generates 3 output pages

    slide.box(show=1).text("One")
    slide.box(show=(1, 1)).text("Two")
    slide.box(show=(1, 2)).text("Three")
    slide.box(show=2).text("Four")
```

Note that if you define that something should be shown (or active) in a step then it is shown (or active) in all of its substeps.
If you want to exclude substeps you have to use `!` character before the step definition:

```nelsie
@deck.slide(debug_steps=True)
def step_demo2(slide):

    slide.set_style("default", TextStyle(size=80))  # Just change size of font

    # This slide generates 3 output pages

    slide.box(show="!1").text("One")
    slide.box(show=(1, 1)).text("Two")
    slide.box(show=(1, 2)).text("Three")
    slide.box(show=2).text("Four")
```


## `last`, `last+`, `next`, `next+` keywords

Box `show` and `active` takes also the following keywords.
By "next major step" we mean the smallest step of the length one that is higher than current visible steps
(e.g. next major step for steps `1, 2, 5` is it `6` and for `2.2, 2.3.1` is it `3`)

* "last" is equivalent to the current highest visible step
* "last+" is equivalent to the current highest visible steps and all steps above
* "next" is equivalent to the next major step
* "next+" is equvalent to the next major step and all steps above 

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