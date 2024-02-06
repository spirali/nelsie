# Getting started

* Create the file `slides.py` with the following content:

```python
from nelsie import SlideDeck

deck = SlideDeck()

@deck.slide()
def hello_world(slide):
    slide.text("Hello world!")
    
deck.render("slides.pdf")
```

* Run `python slides.py`. It will create file `slides.pdf`.