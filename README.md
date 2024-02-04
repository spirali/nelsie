<p align="center">
<img src='docs/imgs/nelsie-logo.jpg' width='400'>
</p>

# Nelsie

Nelsie is a framework for **creating slides programmatically** using Python API with a fast renderer written in Rust.

**Quick links**

- Documentation: TODO
- Demonstration of features ([PDF](https://github.com/spirali/nelsie/blob/demo-rendered/examples/bigdemo/bigdemo.pdf), [source code](examples/bigdemo/bigdemo.py))


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

# Installation from sources

* Install Rust (https://rustup.rs/)
* Install [Maturin](https://www.maturin.rs/) (`pip install maturin`)
* Run in Nelsie source code directory:
  ```commandline
  python3 -m venv venv
  source venv/bin/activate
  maturin build --release
  ```


# History

Nelsie is a complete rewrite of the previous project [Elsie](https://github.com/spirali/elsie). Nelsie solves the biggest pain of Elsie: Dependancy on Inkscape as a renderer engine (It makes difficult to install Elsie on some systems; performance issues and problems when Inkscape changes its programming API). This is solved by a rendering engine shipped within the Nelsie package. Nelsie also offers many improvements in API, namely introduction of `InSteps` and the flexbox layout engine.
