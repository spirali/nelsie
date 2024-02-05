# Nelsie

Nelsie allows you to create slides programmatically using Python. It is a library
with a Python API and a renderer written in Rust.
The output is a PDF file or a set of SVG/PNG files.

There is no DSL or GUI; presentations created with Nelsie are fully programmed in Python.
We believe that creating presentations in a programmable way
makes the process of creating slides smoother and more reliable.

Nelsie focuses on controlling what the audience sees, so you can continuously reveal fragments of the slide,
or simply manage which parts are highlighted.


## Semaphore example

```nelsie
from nelsie import SlideDeck, InSteps

deck = SlideDeck()

@deck.slide()
def semaphore(slide):
    slide.text("Semaphore example", m_bottom=40)

    semaphore = slide.box(width=200, height=600, bg_color="gray")
    semaphore.box(
        y=InSteps({1: 10, 2: 220, 3: 420}),
        width=160,
        height=160,
        bg_color=InSteps({1: "red", 2: "orange", 3: "green"}),
    )
    
deck.render("slides.pdf")  #!IGNORE
```
