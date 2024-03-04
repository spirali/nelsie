from nelsie import TextStyle
from nelsie.helpers.list import ListBox
from testutils import check


@check()
def test_list(deck):
    slide = deck.new_slide(width=200, height=300)
    slide.set_style("default", TextStyle(size=12))
    lst = ListBox(slide)
    lst.text("One")
    lst.text("Two1\nTwo2\nTwo3")
    lst.text("Long item three")
    lst2 = lst.list()
    lst2.text("A")
    lst2.text("BB")
    lst2.text("CCC")
    lst2 = lst.list(list_type="1")
    lst2.text("A")
    lst2.text("BB")
    lst2.text("CCC")
    lst2 = lst.list(list_type="a")
    lst2.text("A")
    lst2.text("BB")
    lst2.text("CCC")
    lst2 = lst.list(list_type="A")
    lst2.text("A")
    lst2.text("BB")
    lst2.text("CCC")
    lst3 = lst2.list()
    lst3.text("Hello!")
    lst.text("Fourth item")
