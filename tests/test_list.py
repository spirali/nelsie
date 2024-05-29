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


@check(n_slides=4)
def test_list_steps(deck):
    slide = deck.new_slide(width=150, height=150)
    slide.set_style("default", TextStyle(size=12))
    lst = ListBox(slide)
    lst.text("One")
    lst.text(show="2+", text="Two1\nTwo2\nTwo3")
    lst.text(show="3+", text="Long item three")
    lst2 = lst.list()
    lst2.text(show="4+", text="A")


@check()
def test_list_initial_value(deck):
    slide = deck.new_slide(width=100, height=50)
    slide.set_style("default", TextStyle(size=12))
    lst = ListBox(slide, list_type="1", initial_counter_value=4)
    lst.text("One")
    lst.text("Two")
    lst.text("Three")
