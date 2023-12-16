import pytest

from nelsie import InSteps, TextStyle


def test_set_style_error(deck):
    with pytest.raises(Exception, match="^Invalid color: 'xxx'$"):
        deck.set_style("abc", TextStyle(color="xxx"))

    with pytest.raises(Exception, match="^Font 'xxx' not found.$"):
        deck.set_style("abc", TextStyle(font_family="xxx"))

    with pytest.raises(Exception, match="^Invalid color: 'xxx'$"):
        deck.set_style("abc", InSteps({1: TextStyle(), 4: TextStyle(color="xxx")}))


def test_invalid_box(deck):
    with pytest.raises(Exception, match="^Invalid value: xxx$"):
        deck.new_slide().box(width=InSteps({1: 10, 2: "xxx"}))

    with pytest.raises(Exception, match="^Invalid color: 'xxx'$"):
        deck.new_slide().box(bg_color="xxx")


# TODO
# def test_invalid_text_style(deck):
#     deck.new_slide().text("Text", style=TextStyle(color="xxx"))
