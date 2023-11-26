from nelsie import SlideDeck

deck = SlideDeck()


@deck.slide()
def hello_world(slide):
    slide.text("Hello world!")


deck.render(output_pdf="minimal.pdf")
