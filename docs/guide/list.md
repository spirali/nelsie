# List

Lists are not build-in in Nelsie. You can create lists manually, or you can create
a helper class `ListBox` for creating lists.

`ListBox` takes a box as a first parameter and creates list in this box.
`ListBox` behaves as a normal vertical box but creates a bullet for each child.

## Unordered list

```nelsie
from nelsie.helpers.list import ListBox

@deck.slide()
def list_demo(slide):
    slide.set_style("default", TextStyle(size=80))
    lst = ListBox(slide)
    lst.text("First item")
    lst.text("Second item")
    lst.text("Third item")
```

## Ordered list

You can change type of by setting a second argument `list_type` to following values:

- `"unordered"` (default) - Unordered list
- `"1"` - Ordered list, 1., 2., 3. ...
- `"a"` - Ordered list, a., b., c. ...
- `"A"` - Ordered list, A., B., C. ...

```nelsie
from nelsie.helpers.list import ListBox

@deck.slide()
def list_demo(slide):
    slide.set_style("default", TextStyle(size=80))
    lst = ListBox(slide, "1")
    lst.text("First item")
    lst.text("Second item")
    lst.text("Third item")
```


```nelsie
from nelsie.helpers.list import ListBox

@deck.slide()
def list_demo(slide):
    slide.set_style("default", TextStyle(size=80))
    lst = ListBox(slide, "a")
    lst.text("First item")
    lst.text("Second item")
    lst.text("Third item")
```


## Sublists

A sublist can be created by calling `.list()` method on an existing `ListBox`.
It returns an instance of `ListBox`.

```nelsie
from nelsie.helpers.list import ListBox

@deck.slide()
def list_demo(slide):
    slide.set_style("default", TextStyle(size=80))

    lst = ListBox(slide)
    lst.text("First item")
    lst.text("Second item")

    lst2 = lst.list()
    lst2.text("Hello")
    lst2.text("World!")
```