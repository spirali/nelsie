from testutils import check
from dataclasses import asdict

from nelsie import InSteps, TextStyle

import pytest


def test_text_update():
    s1 = TextStyle(color="green", size=123)
    s2 = TextStyle(size=321, line_spacing=1.5)
    s3 = s1.update(s2)
    assert s3.color == "green"
    assert s3.size == 321
    assert s3.line_spacing == 1.5


def test_text_invalid_font(deck):
    s1 = TextStyle(font_family="Nonexisting font")
    with pytest.raises(Exception, match="Font 'Nonexisting font' not found"):
        deck.new_slide().text("Hello", style=s1)


@check(n_slides=4)
def test_render_text_basic(deck):
    deck.set_style("highlight", TextStyle(color="orange"))
    slide = deck.new_slide()
    slide.set_style("small", TextStyle(size=8))
    slide.box(bg_color="#f88").text(
        "Hello ~highlight{world! ~small{this is small}}. End of text.\nNew line\nThird line"
    )
    slide = deck.new_slide()
    slide.box(bg_color="#f88").text("A\n\nBB")

    slide = deck.new_slide()
    slide.box(bg_color="#f88").text("\nLines up & below\n\n\n")

    slide = deck.new_slide()
    slide.set_style("big", TextStyle(size=64))
    slide.box(bg_color="#f88").text(
        "Now follows: ~big{Big text}\nNext line\nNext line\nNext line"
    )


@check(n_slides=2)
def test_render_text_steps(deck):
    deck.set_style("highlight", TextStyle(color="orange"))
    slide = deck.new_slide(width=300, height=100)
    slide.set_style(
        "my_style",
        InSteps({1: TextStyle(color="green"), 2: TextStyle(color="orange", size=64)}),
    )
    #    slide.box(bg_color="#f88").text("Say ~my_style{hello}!")
    slide.box().text("Say ~my_style{hello}!")


@check()
def test_render_text_unicode(deck):
    deck.set_style("x", TextStyle(color="yellow"))
    slide = deck.new_slide()
    slide.box(bg_color="#f88").text(
        "Příliš žluťoučký ~x{kůň} úpěl ďábelské ódy\n>>>y̆<<<"
    )


def test_set_invalid_font(deck):
    with pytest.raises(Exception, match="Font 'NON-existent-fnt' not found."):
        deck.set_style("my_style", TextStyle(font_family="NON-existent-fnt"))


def test_set_get_styles_deck(deck):
    s = deck.get_style("default")
    for key, value in asdict(s).items():
        assert value is not None

    assert s.font_family in ("DejaVu Sans", "Arial")
    assert s.color == "#000000"
    assert s.size == pytest.approx(32.0)
    assert s.line_spacing == pytest.approx(1.2)

    with pytest.raises(Exception, match="Style 'big' not found"):
        deck.get_style("big")

    deck.set_style("big", TextStyle(size=120.0))
    s = deck.get_style("big")
    for key, value in asdict(s).items():
        if key != "size":
            assert value is None
    assert s.size == pytest.approx(120.0)


def test_set_get_styles_box(deck):
    slide = deck.new_slide()
    slide.set_style("one", TextStyle(color="red"))
    slide.set_style("two", TextStyle(color="green"))
    b = slide.box()
    b2 = b.box()
    b2.set_style("one", TextStyle(color="blue"))
    b2.set_style("three", TextStyle(size=321))
    b2.set_style("four", InSteps({1: TextStyle(size=100), 4: TextStyle(size=40)}))
    b3 = b2.box()
    b3.set_style("default", TextStyle(line_spacing=1.0))

    with pytest.raises(Exception, match="Style 'three' not found"):
        deck.get_style("three")
    with pytest.raises(Exception, match="Style 'three' not found"):
        b.get_style("three")

    s = b.get_style("one")
    assert s == TextStyle(color="#ff0000")

    s = b3.get_style("one")
    assert s == TextStyle(color="#0000ff")

    s = b3.get_style("three")
    assert s.size == pytest.approx(321.0)

    s = b3.get_style("default")
    assert s.line_spacing == 1.0
    assert s.color == "#000000"
