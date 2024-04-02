# Output formats

This section shows you how to get slides in PDF, SVG or PNG and how to render slides into files or
how to get them as Python objects.

## Rendering into PDF

By default, the `.render()` method on a slide deck takes a filename and creates a PDF file.

```python
from nelsie import SlideDeck

deck = SlideDeck()

# Add slides here

deck.render("slides.pdf")
```

## Rendering into SVG or PNG

By setting the second parameter to "svg" or "png", you can change the output format to SVG or PNG.

```python
from nelsie import SlideDeck

deck = SlideDeck()

# Add slides here

deck.render("output/path1", "svg")  # Render slides to SVG

deck.render("output/path2", "png")  # Render slides to PNG
```

Unlike PDF, the first parameter is not a path to a file, but to a directory where
Nelsie creates SVG (or PNG) images, one image per slide page.
Nelsie will create the target directory if it does not exist.
Images are named in the format "X-Y-Z.svg" (or "X-Y-Z.png"), where X is the page index (zero padded), Y is the slide
index and Z is a step.

## In-memory rendering

If the first parameter of the `.render()` method is `None` then Nelsie does not create files but returns
the images as Python objects. It returns a list of triplets (`slide_id`, `step`, `data`) where `data` are
`bytes` instance with the image.

```python
pages = deck.render(None, "png")

print(pages)  # Print returned triplets with pages
```