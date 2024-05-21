# Links

You can create a clicable link in slides as follows:

```nelsie
@deck.slide()
def three_boxes(slide):
    slide.text("Nelsie",
               TextStyle(size=64, underline=True),
               url="https://github.com/spirali/nelsie")
```

If the slides are rendered into PDF it will create a clicable link.


!!! note

    Creating clicable links now works only if you create PDF file as the output. For SVG and PNG output it is not supported. Therefore it is also not working here in the documentation.

## `url` parameter of a box

Parameter `url` can be passed to any box, not only text as shows the example above. Therefore, you can create a clicable link from images or a set of other boxes, or just part of the text if you use a text anchor box.

Example:

```python
slide.image("logo.png", url="...")

box = slide.box(url="...")
box.image(...)
box.text(...)
```