<p align="center">
<img src='docs/imgs/nelsie-logo.jpg' width='400'>
</p>

# Nelsie

Nelsie is a framework for **creating slides programmatically** using Python with a fast renderer written in Rust.

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

deck.render(output_pdf="slides.pdf")
```