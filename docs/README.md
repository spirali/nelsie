# Nelsie

Nelsie allows you to create slides programmatically using Python. It is a library
with a Python API and a renderer written in Rust.
The output is a PDF file or a set of SVG/PNG files.

There is no DSL or GUI; presentations created with Nelsie are fully programmed in Python.
We believe that creating presentations in a programmable way
makes the process of creating slides smoother and more reliable.

Nelsie focuses on controlling what the audience sees, so you can continuously reveal fragments of the slide,
or simply manage which parts are highlighted.


## Minimal example

```python
import elsie

slides = elsie.SlideDeck()

@slides.slide()
def hello(slide):
    slide.text("Hello world!")

slides.render("slides.pdf")
```
