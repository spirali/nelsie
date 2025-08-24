import pytest

from nelsie import TextStyle, StepVal


def test_set_style_error(deck):
    with pytest.raises(Exception, match="^Invalid color: 'xxx'$"):
        deck.set_style("abc", TextStyle(color="xxx"))

    with pytest.raises(Exception, match="^Invalid color: 'xxx'$"):
        deck.set_style("abc", StepVal(TextStyle()).at(4, TextStyle(color="xxx")))


def test_invalid_box(deck):
    with pytest.raises(Exception, match="^Invalid size definition"):
        deck.new_slide().box(width=StepVal(10).at(2, "xxx"))

    with pytest.raises(Exception, match="^Invalid color: 'xxx'$"):
        deck.new_slide().box(bg_color="xxx")


# TODO
# def test_invalid_text_style(deck):
#     deck.new_slide().text("Text", style=TextStyle(color="xxx"))
