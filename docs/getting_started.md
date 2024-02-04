# Getting started


## Step by step



## Minimal example

```nelsie
from nelsie import SlideDeck

deck = SlideDeck()

@deck.slide()
def hello_world(slide):
    slide.text("Hello world!")

deck.render(output_pdf="slides.pdf")
```
