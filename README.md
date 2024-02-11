
<p align="center">
<img src='docs/imgs/nelsie-logo.jpg' width='400'>
</p>

# Nelsie

[![Build](https://github.com/spirali/nelsie/actions/workflows/build.yaml/badge.svg?branch=main)](https://github.com/spirali/nelsie/actions/workflows/build.yaml)

Nelsie is a framework for **creating slides programmatically** using Python API with a fast renderer written in Rust.

**Quick links**

- [Documentation](https://spirali.github.io/nelsie/) [status: in progress]
- Demonstration of features ([PDF](https://spirali.github.io/nelsie/pdf/bigdemo.pdf), [source code](examples/bigdemo/bigdemo.py))


## Hello world

```python
from nelsie import SlideDeck

deck = SlideDeck()

@deck.slide()
def hello_world(slide):
    slide.text("Hello world!")

deck.render("slides.pdf")
```

# Installation

```commandline
$ pip install nelsie
```
