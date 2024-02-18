
# Resources

An instance of the Resources class holds information about fonts and loaded images.
By default, an instance of SlideDeck creates its own instance of Resources,
but you can create your own instance and pass it to the SlideDeck constructor.

There are two main scenarios where it is useful to create your own instance of `Resources'.

* You want to register your own fonts
* You create more instances of `SlideDeck` and you want to skip some initialization related to font detection or to skip loading the same images repeatedly.

`Resources` instance can also provide a list of available syntaxes and syntax highlighting themes.

## Example: Registering own fonts

```python
from nelsie import Resources, SlideDeck

resources = Resources()
resources.load_fonts_dir("path/to/fonts")

deck = SlideDeck(resources=resources)
```


## Example: Reusing resources in more slide decks

```python
from nelsie import Resources, SlideDeck

resources = Resources()

deck1 = SlideDeck(resources=resources)
deck2 = SlideDeck(resources=resources)
```

## List of syntaxes

```python
from nelsie import Resources

resources = Resources()
print(resources.syntaxes())
```

## List of themes for syntax highlighting

```python
from nelsie import Resources

resources = Resources()
print(resources.themes())
```
