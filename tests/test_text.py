from nelsie.text.manager import TextStyleManager
from nelsie.text.parse import parse_styled_text
from nelsie.text.textstyle import DEFAULT_STYLE
from nelsie.text.texttypes import StyledLine, StyledSpan
from testutils import check

from nelsie import InSteps, TextStyle


def test_text_update():
    s1 = TextStyle(color="green", size=123)
    s2 = TextStyle(size=321, line_spacing=1.5)
    s3 = s1.update(s2)
    assert s3.color == "green"
    assert s3.size == 321
    assert s3.line_spacing == 1.5


@check(error=Exception, error_match="Font 'Nonexisting font' not found.")
def test_text_invalid_font(deck):
    s1 = TextStyle(font_family="Nonexisting font")
    deck.new_slide().text("Hello", style=s1)


def test_text_style_manager_no_steps():
    manager = TextStyleManager({"default": DEFAULT_STYLE})
    manager.set_style("red", TextStyle(color="red", size=123))

    assert manager.get_style("red") == TextStyle(color="red", size=123)
    assert manager.get_final_style("red") == DEFAULT_STYLE.update(
        TextStyle(color="red", size=123)
    )

    manager.update_style("red", TextStyle(color="green"))
    assert manager.get_style("red") == TextStyle(color="green", size=123)

    manager.set_style("red", TextStyle(color="blue"))
    assert manager.get_style("red") == TextStyle(color="blue")

    manager2 = manager.copy()
    manager2.set_style("red", TextStyle(color="orange"))
    assert manager.get_style("red") == TextStyle(color="blue")
    assert manager2.get_style("red") == TextStyle(color="orange")


def test_text_style_manager_steps():
    manager = TextStyleManager({"default": DEFAULT_STYLE})
    manager.set_style("red", TextStyle(color="red", size=123))
    manager.update_style(
        "red", InSteps({1: TextStyle(size=15), 2: TextStyle(), 3: TextStyle(size=15)})
    )
    manager.update_style("red", InSteps({1: TextStyle(), 4: TextStyle(color="green")}))
    assert manager.get_style("red").in_step_values == {
        1: TextStyle(color="red", size=15),
        2: TextStyle(color="red", size=123),
        3: TextStyle(color="red", size=15),
        4: TextStyle(color="green", size=15),
    }


def test_parse_text():
    style = DEFAULT_STYLE.update(TextStyle(size=10, line_spacing=1.2))
    style_manager = TextStyleManager({"default": DEFAULT_STYLE})
    name_style = TextStyle(color="red")
    style_manager.set_style("name", name_style)
    full_name_style = style.update(name_style)

    style_manager.set_style("l1", TextStyle(color="green"))
    l1 = style_manager.get_style("l1")
    style_manager.set_style("l2", TextStyle(size=20, line_spacing=1.3))
    l2 = style_manager.get_style("l2")
    style_manager.set_style("l3", TextStyle(size=25, color="orange"))
    l3 = style_manager.get_style("l3")

    st = parse_styled_text("Hello!", "~{}", style, style_manager).in_step_values[1]
    assert st.default_font_size == 10
    assert st.default_line_spacing == 1.2
    assert st.styled_lines == [StyledLine(text="Hello!", spans=[StyledSpan(0, 6, 0)])]
    assert st.styles == [style]

    st = parse_styled_text("~~~~~~", "~{}", style, style_manager).in_step_values[1]
    assert st.styled_lines == [StyledLine(text="~~~", spans=[StyledSpan(0, 3, 0)])]
    assert st.styles == [style]

    st = parse_styled_text("a\n\nbb\nccc", "~{}", style, style_manager).in_step_values[1]
    assert st.styled_lines == [
        StyledLine(text="a", spans=[StyledSpan(0, 1, 0)]),
        StyledLine(text="", spans=[]),
        StyledLine(text="bb", spans=[StyledSpan(0, 2, 0)]),
        StyledLine(text="ccc", spans=[StyledSpan(0, 3, 0)]),
    ]
    assert st.styles == [style]

    st = parse_styled_text("\na\nb\n", "~{}", style, style_manager).in_step_values[1]
    assert st.styled_lines == [
        StyledLine(text="", spans=[]),
        StyledLine(text="a", spans=[StyledSpan(0, 1, 0)]),
        StyledLine(text="b", spans=[StyledSpan(0, 1, 0)]),
        StyledLine(text="", spans=[]),
    ]
    assert st.styles == [style]

    st = parse_styled_text("{Alice}", "~{}", style, style_manager).in_step_values[1]
    assert st.styled_lines == [StyledLine(text="{Alice}", spans=[StyledSpan(0, 7, 0)])]
    assert st.styles == [style]

    st = parse_styled_text("~name{Alice}", "~{}", style, style_manager).in_step_values[1]
    assert st.styled_lines == [StyledLine(text="Alice", spans=[StyledSpan(0, 5, 0)])]
    assert st.styles == [full_name_style]

    st = parse_styled_text(
        "My name is ~name{Alice}\n~name{Bob} is your name.", "~{}", style, style_manager
    ).in_step_values[1]

    assert st.styled_lines == [
        StyledLine(
            #     0123456789012345
            text="My name is Alice",
            spans=[StyledSpan(0, 11, 0), StyledSpan(11, 5, 1)],
        ),
        StyledLine(
            text="Bob is your name.",
            spans=[StyledSpan(0, 3, 1), StyledSpan(3, 14, 0)],
        ),
    ]
    assert st.styles == [style, full_name_style]

    st = parse_styled_text(
        "L0~l1{L1~l2{L2~l3{L3}}}L0", "~{}", style, style_manager
    ).in_step_values[1]
    assert st.styled_lines == [
        StyledLine(
            text="L0L1L2L3L0",
            spans=[
                StyledSpan(0, 2, 0),
                StyledSpan(2, 2, 1),
                StyledSpan(4, 2, 2),
                StyledSpan(6, 2, 3),
                StyledSpan(8, 2, 0),
            ],
        ),
    ]

    l_styles = [
        style,
        style.update(l1),
        style.update(l1).update(l2),
        style.update(l1).update(l2).update(l3),
    ]
    assert st.styles == l_styles

    st = parse_styled_text(
        "L0~l1{L\n1~l2{\nL2~l3{L3}}}L0", "~{}", style, style_manager
    ).in_step_values[1]
    assert st.styled_lines == [
        StyledLine(text="L0L", spans=[StyledSpan(0, 2, 0), StyledSpan(2, 1, 1)]),
        StyledLine(text="1", spans=[StyledSpan(0, 1, 1)]),
        StyledLine(
            text="L2L3L0",
            spans=[StyledSpan(0, 2, 2), StyledSpan(2, 2, 3), StyledSpan(4, 2, 0)],
        ),
    ]
    assert st.styles == l_styles


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
    slide.box(bg_color="#f88").text("\nLines up & below\n\n")

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