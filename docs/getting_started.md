# Getting started

* Install Nelsie package:
    ```commandline
    $ pip install nelsie
    ```

* Create the file `slides.py` with the following content:

    ```python
    from nelsie import SlideDeck

    # Create a slide deck
    deck = SlideDeck()

    # Insert a slide
    @deck.slide()
    def hello_world(slide):
        slide.text("Hello world!")

    # Render into PDF
    deck.render("slides.pdf")
    ```

* Run `python slides.py`. It creates file `slides.pdf`.

If you do not want to build your script manually, you can use [Automatic slide rebuilding](guide/watch.md)