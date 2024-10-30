# Resources

An instance of the Resources class holds information about fonts, code syntaxes and themes, and loaded images.
By default, an instance of SlideDeck creates its own instance of Resources,
but you can create your own instance and pass it to the SlideDeck constructor.

There are two main scenarios where it is useful to create your own instance of `Resources'.

* You want to register your own fonts, code syntaxes, or code themes
* You create more instances of `SlideDeck` and you want to skip some initialization or to skip
  loading the same images repeatedly.

`Resources` instance can also provide a list of available syntaxes and syntax highlighting themes.

## Registering own fonts

```python
from nelsie import Resources, SlideDeck

resources = Resources()
resources.load_fonts_dir("path/to/fonts")

deck = SlideDeck(resources=resources)
```

## Loading system fonts

By default, Nelsie loads a fonts shipped together with Nelsie and ignores system fonts.
You can change this by the following command command and load system fonts from your system.

```python
from nelsie import Resources

resources = Resources(system_fonts=True, builtin_fonts=False)
```

!!! warning

  The current text rendering engine used in Nelsie tries to preload some font data. So loading system fonts may take some time. On normal system it should be around 0.1s, but if you have many fonts installed it may be significantly slower. The solution is to create
  a directory only with fonts that you are using in your slides and do not load all system fonts in such situations. It would also make
  your slides more replicable on other computers.

## Loading custom code syntaxes

Nelsie supports loading syntax files from Sublime editor (files with `.sublime-syntax` extension).

```python
from nelsie import Resources, SlideDeck

resources = Resources(default_code_syntaxes=False)
resources.load_code_syntax_dir("path/to/syntaxes")

deck = SlideDeck(resources=resources)
```

!!! warning "Known bug"

    If you want to add custom syntax definitions, you have to disable loading default
    syntaxes (`default_code_syntaxes=False`)
    otherwise `.load_code_syntax_dir()` will not work.

## Loading custom code color themes

Nelsie supports loading color theme `thTheme` (files with `.thTheme` extension).

```python
from nelsie import Resources, SlideDeck

resources = Resources()
resources.load_code_theme_dir("path/to/themes")

deck = SlideDeck(resources=resources)
```

## Reusing resources in more slide decks

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
