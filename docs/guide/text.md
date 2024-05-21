# Text

This section is about drawing text on slides.

Text is drawn on a box (or slide) by calling the `.text()` method. It creates a new box containing a text.

```nelsie

@deck.slide()
def text_demo(slide):
    slide.text("Hello world!")
```

!!! note "Note for Elsie users"

    Calling `.text()` creates a new box; this is a different behavior than in Elsie, where calling `.text()` does not create a new box, which very often leads to code like `.box().text()` to create a wrapping box. This is not necessary in Nelsie.


## Text styles

The drawing of a text is configured by `TextStyle` instances.
One of the uses is to set it as the second argument of the `.text()` method.

```nelsie
from nelsie import TextStyle

@deck.slide()
def text_style_demo(slide):
    style = TextStyle(size=100, color="green")
    slide.text("Hello world!", style)
```

The `TextStyle` constructor has the following parameters; each parameter can be `None', which means that the parameter will not be overridden by this style.

* `font_family`: `str` - Name of the font
* `color`: `str` - Color of the text
* `size`: `float` - Size of the font
* `line_spacing`: `float` - Line spacing relative to `size`
* `italics`: `bool` - Enable italic mode
* `weight`: `int` - Weight of the font; values 1-1000
    * 400 = Normal
    * 700 = Bold
* `underline`: `bool` - Draws a line under the text
* `overline`: `bool` - Draws a line over the text
* `line_through`: `bool` - Draws a line through a text
* `stroke`: `Stroke | None` - If not `None`, font is drawn in stroked mode (see [Paths](paths.md) for documentation of `Stroke` class)
* `stretch`: `FontStretch`:
    * `FontStretch.UltraCondensed`
    * `FontStretch.ExtraCondensed`
    * `FontStretch.Condensed`
    * `FontStretch.SemiCondensed`
    * `FontStretch.Normal`
    * `FontStretch.SemiExpanded`
    * `FontStretch.Expanded`
    * `FontStretch.ExtraExpanded`
    * `FontStretch.UltraExpanded`

## Named styles

Each box can have a set of named fonts defined. When a new box is created
it inherits all named styles from its parent.

```nelsie
@deck.slide()
def text_style_demo(slide):
    slide.set_style("my-style", TextStyle(size=100, color="red"))
    slide.text("Hello world!", "my-style")
```

Slide deck may also defines a font that is inherited by all slides.

```python
deck.set_slide("my-style", TextStyle(size=100, color="red"))
```

## Build-in styles

There are three predefined text styles:

* `"default"`
* `"monospace"`
* `"code"`

Style `"default"` is special and is used as a source of default values for drawing fonts when values are not overridden by more specific fonts:


```nelsie
@deck.slide()
def default_style_demo(slide):

    # Set default style
    slide.set_style("default", TextStyle(color="blue"))

    # Draw text with overriden style, color is taken from the default style
    slide.text("Hello world!", TextStyle(size=100))
```

Style `"monospace"` sets the font family to a monospace font.

Style `"code"` is used as a default style in `.code()` method. See [Code](code.md) for more details. By default is have the same effect as style `"monospace"`.


## Inline styles

Named styles are particularly useful for modifying individual blocks of text within a single string passed to the `.text()` method. To style a block of text, use the following syntax `~STYLE{TEXT}` where STYLE is a style name and TEXT is the styled text.

```nelsie
@deck.slide()
def inline_style_demo(slide):

    slide.set_style("red", TextStyle(color="red"))
    slide.set_style("big", TextStyle(size=64))

    slide.text("~red{Hello} world!\n~monospace{github.com/spirali/~big{nelsie}}")
```


## Fonts

A font can be specified by the `font_family` parameter of `TextStyle`.
All system fonts are available by default. You can add more fonts via [Resources](resources.md).

Nelsie is not shipped with a built-in font and tries to automatically detect a sans-serif font as `font_family` for the `"default"` style and a monospace font for the `"monospace"` style.

You can override this behavior by setting

```python
deck = SlideDeck(default_font="Helvetica", default_monospace_font="Ubuntu Mono")
```


!!! note "Robust slide rendering across systems"

    For robust cross-platform slide rendering, it is recommended to include all used fonts along with the slide source code.


## Text alignment

A text can be aligned to the left, center, and right by setting `.text(align="...")` to `"start"`, `"center"`, or `"end"`. The value `"start"` is the default.

```nelsie
@deck.slide()
def text_align_demo(slide):

    TEXT = "Line 1\nLooooong line\nThird line"

    box = slide.box(gap=(0, 50))
    box.text(TEXT, align="start")
    box.text(TEXT, align="center")
    box.text(TEXT, align="end")
```


## Text box

Calling `.text()` creates a box for the text; the method takes the same arguments as `.box()` to configure the underlying box.

```nelsie
@deck.slide()
def text_box_demo(slide):
    box = slide.box(bg_color="gray")
    box.text("Hello world!", bg_color="orange", m_x=50, m_y=30)
```

## Updating style

The `.set_style()` method overrides the whole style over given name:

```nelsie
deck.set_style("my-style", TextStyle(color="green"))

@deck.slide()
def text_style_demo(slide):
    slide.set_style("default", TextStyle(size=80))

    # "my-style" now forgets the color, as we fully redefining what "my-style" is
    slide.set_style("my-style", TextStyle(italic=True))
    slide.text("Hello world!", "my-style")
```

There is method `.update_style()`, if we want to "update" style, and change only some properties and keep others.

```nelsie
deck.set_style("my-style", TextStyle(color="green"))

@deck.slide()
def text_style_demo(slide):
    slide.set_style("default", TextStyle(size=80))

    # "my-style" now contains both color change to green and italic style
    slide.update_style("my-style", TextStyle(italic=True))
    slide.text("Hello world!", "my-style")
```


!!! note "Setting a default style"

    There is an exception for style `"default"` as it always needs to define all attributes.
    Hence `.set_style()` for `"default"` style always behaves as `.update_style()`.


## Text and `InSteps`

You may use `InSteps` in `.text()`:

```nelsie
@deck.slide()
def text_style_demo(slide):
    slide.set_style("default", TextStyle(size=80))
    slide.text(InSteps({1: "Hello world!", 2: "Hello Nelsie!"}))
```

You can also provide an array of strings and `InSteps`. String in the array is concatenated for each step:

```nelsie
@deck.slide()
def text_style_demo(slide):
    slide.set_style("default", TextStyle(size=80))
    slide.text(["Hello ", InSteps({1: "world", 2: "Nelsie"}), "!"])
```


## Text styles and `InSteps`

When a style is set through `set_style` an instance of `InSteps` can be used:

```nelsie
@deck.slide()
def text_style_demo(slide):
    slide.set_style("default", TextStyle(size=80))
    slide.set_style("my-style",
                    InSteps({1: TextStyle(color="red"), 2: TextStyle(color="green")}))
    slide.text("Hello world!", "my-style")
```


## Automatic text stripping

The `.text()` method automatically strips whitespace from the beginning and end of the text.
This can be disabled by setting `.text(..., strip=False)`.