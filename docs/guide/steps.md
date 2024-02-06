
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
