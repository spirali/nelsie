from dataclasses import asdict

import pytest
from testutils import check

import nelsie
from nelsie import FontStretch, InSteps, Stroke, TextStyle


def test_text_update():
    s1 = TextStyle(color="green", size=123)
    s2 = TextStyle(size=321, line_spacing=1.5)
    s3 = s1.merge(s2)
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
    slide.box(bg_color="#f88").text("\nLines up & below\n\n\n", strip=False)

    slide = deck.new_slide()
    slide.set_style("big", TextStyle(size=64))
    slide.box(bg_color="#f88").text("Now follows: ~big{Big text}\nNext line\nNext line\nNext line")


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
    slide.box(bg_color="#f88").text("Příliš žluťoučký ~x{kůň} úpěl ďábelské ódy\n>>>y̆<<<")


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


# On windows, underline is drawn in slightly different place, I do not know why
@check(windows_threshold=1800)
def test_text_lines(deck):
    slide = deck.new_slide(width=200, height=250)
    slide.text("Test text", TextStyle(underline=True), m_bottom=10)
    slide.text("Test text", TextStyle(overline=True), m_bottom=10)
    slide.text("Test text", TextStyle(line_through=True), m_bottom=10)
    slide.text("Test text", TextStyle(underline=True, overline=True, line_through=True))


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


@check()
def test_text_color_opacity(deck):
    slide = deck.new_slide(width=220, height=50)
    slide.set_style("one", TextStyle(color="#ff00ff50"))
    slide.box(x=0, y=0, width=60, height="100%", bg_color="green")
    slide.box(x=60, y=0, width="30%", height="100%", bg_color="blue")
    slide.text("Opacity test", style="one")
    assert slide.get_style("one").color == "#ff00ff50"


@check()
def test_text_styling(deck):
    slide = deck.new_slide(width=220, height=150)
    slide.text("Italic", style=TextStyle(italic=True))
    slide.text("Bold", TextStyle(weight=700))
    slide.text("Hi are you?", TextStyle(stretch=FontStretch.UltraCondensed, size=10))
    slide.text("Hi are you?", TextStyle(stretch=FontStretch.UltraExpanded, size=10))


def test_text_style_get_stretch(deck):
    slide = deck.new_slide(width=220, height=150)
    slide.set_style("test", TextStyle(stretch=FontStretch.Expanded))
    assert isinstance(slide.get_style("test").stretch, nelsie.FontStretch)
    assert slide.get_style("default").stretch == FontStretch.Normal
    assert slide.get_style("test").stretch == FontStretch.Expanded


@check()
def test_text_stroke(deck):
    slide = deck.new_slide(width=150, height=100, bg_color="orange")
    slide.text("Text 1", style=TextStyle(stroke=Stroke("green"), color="#909090"))
    slide.text(
        "Text 2",
        style=TextStyle(stroke=Stroke("blue", dash_array=[5, 2], width=0.2), color="empty"),
    )


def test_text_style_get_stroke(deck):
    slide = deck.new_slide(width=220, height=150)
    slide.set_style(
        "test",
        TextStyle(stroke=Stroke("green", dash_array=[5, 2], width=10, dash_offset=2)),
    )
    assert slide.get_style("default").stroke == "empty"
    assert slide.get_style("test").stroke == Stroke("#008000", dash_array=[5, 2], width=10, dash_offset=2)


@check()
def test_text_monospace(deck):
    slide = deck.new_slide(width=150, height=100)
    slide.text("Text W1", "monospace")


@check()
def test_text_align(deck):
    slide = deck.new_slide(width=100, height=130)
    slide.set_style("default", TextStyle(size=12))
    slide.text("Hello world\nNow!\nand there", align="start", bg_color="red")
    slide.text(
        "Hello world\nNow!\nand there",
        style=TextStyle(color="green"),
        align="center",
        bg_color="gray",
    )
    slide.text(
        "Hello world\nNow!\nand there",
        style=TextStyle(color="blue"),
        align="end",
        bg_color="#990099",
    )


@check()
def test_text_descent_ascent1(deck):
    slide = deck.new_slide(width=200, height=280)
    slide.set_style("default", TextStyle(size=24))
    box = slide.box(row=True)
    box.text("W1yg", bg_color="green", style=TextStyle(line_spacing=1.0))
    box.text("W1yg", bg_color="red", style=TextStyle(line_spacing=1.2))

    box = slide.box(row=True, bg_color="gray")
    box.text("Wy\nWy", bg_color="green", style=TextStyle(line_spacing=1.0))
    box.text("Wy\nWy", bg_color="cyan", style=TextStyle(line_spacing=1.5))

    slide.set_style("a", TextStyle(size=6))
    slide.text("Wg~a{Wg}~monospace{Wg}", bg_color="orange")

    slide.set_style("b", TextStyle(line_spacing=1.5))
    slide.text("Wg\nWg\n~b{Wg\n~a{Wg\nWg}}\nWg", bg_color="gray")


@check()
def test_text_descent_ascent2(deck):
    slide = deck.new_slide(width=20, height=380)
    slide.set_style("default", TextStyle(size=12, line_spacing=1.5))
    slide.text("g\n" * 20, bg_color="red")


@check()
def test_line_box(deck):
    deck.set_style("default", TextStyle(size=8))
    deck.set_style("big", TextStyle(size=18))
    slide = deck.new_slide(width=120, height=100)
    t = slide.text("First line\n\n~big{Third} line\n4. line\nLooooooooong line", z_level=1)

    slide.box(
        x=t.line_x(0),
        y=t.line_y(0),
        width=t.line_width(0),
        height=t.line_height(0),
        bg_color="green",
    )
    slide.box(
        x=t.line_x(1),
        y=t.line_y(1),
        width=t.line_width(1),
        height=t.line_height(1),
        bg_color="blue",
    )
    slide.box(
        x=t.line_x(2),
        y=t.line_y(2),
        width=t.line_width(2),
        height=t.line_height(2),
        bg_color="red",
    )
    slide.box(
        x=t.line_x(4),
        y=t.line_y(4),
        width=t.line_width(4),
        height=t.line_height(4),
        bg_color="orange",
    )


@check()
def test_text_line_points1(deck):
    deck.set_style("default", TextStyle(size=12))
    deck.set_style("big", TextStyle(size=18))
    slide = deck.new_slide(width=150, height=130)
    t1 = slide.text("x\nyy\nzzzzz", z_level=1, align="start")
    t2 = slide.text("x\nyy\nzzzzz", z_level=1, align="center")
    t3 = slide.text("x\nyy\nzzzzz", z_level=1, align="end")

    for t in (t1, t2, t3):
        slide.box(
            x=t.line_x(0),
            y=t.line_y(0),
            width=t.line_width(0),
            height=t.line_height(0),
            bg_color="green",
        )
        slide.box(
            x=t.line_x(1),
            y=t.line_y(1),
            width=t.line_width(1),
            height=t.line_height(1),
            bg_color="blue",
        )
        slide.box(
            x=t.line_x(2),
            y=t.line_y(2),
            width=t.line_width(2),
            height=t.line_height(2),
            bg_color="red",
        )


@check(n_slides=3)
def test_text_anchor_points(deck):
    deck.set_style("default", TextStyle(size=12))
    deck.set_style("green", TextStyle(size=16, color="green"))

    for align in ("start", "center", "end"):
        slide = deck.new_slide(width=180, height=100)
        t = slide.text(
            "Hello ~green{~1{world!}}\n~2{Full line}\n>~100{1}<<<",
            z_level=1,
            align=align,
        )
        slide.box(
            x=t.text_anchor_x(1),
            y=t.text_anchor_y(1),
            width=t.text_anchor_width(1),
            height=t.text_anchor_height(1),
            bg_color="orange",
        )
        slide.box(
            x=t.text_anchor_x(2),
            y=t.text_anchor_y(2),
            width=t.text_anchor_width(2),
            height=t.text_anchor_height(2),
            bg_color="gray",
        )
        slide.box(
            x=t.text_anchor_x(100),
            y=t.text_anchor_y(100),
            width=t.text_anchor_width(100),
            height=t.text_anchor_height(100),
            bg_color="cyan",
        )


@check()
def test_text_line_points2(deck):
    deck.set_style("default", TextStyle(size=12))
    deck.set_style("green", TextStyle(size=46, color="green"))

    slide = deck.new_slide(width=250, height=150)
    t = slide.text("Hello ~green{yWy!}\nFull line gy\nWgy", z_level=1)
    slide.box(
        x=t.line_x(0),
        y=t.line_y(0),
        width=t.line_width(0),
        height=t.line_height(0),
        bg_color="orange",
    )
    slide.box(
        x=t.line_x(1),
        y=t.line_y(1),
        width=t.line_width(1),
        height=t.line_height(1),
        bg_color="blue",
    )
    slide.box(
        x=t.line_x(2),
        y=t.line_y(2),
        width=t.line_width(2),
        height=t.line_height(2),
        bg_color="orange",
    )


@check()
def test_text_boxes(deck):
    deck.set_style("default", TextStyle(size=12))

    slide = deck.new_slide(width=50, height=50)
    t = slide.text("Line 1\nL~123{ine} 2!", z_level=1)

    t.line_box(1, bg_color="green", z_level=0)
    t.text_anchor_box(123, bg_color="orange", z_level=0)


@check()
def test_text_anchors_space_prefix(deck):
    deck.update_style("code", TextStyle(size=22))
    deck.set_style("s1", TextStyle(color="green"))
    slide = deck.new_slide(width=560, height=160)
    t = slide.text(
        """                                            ~1{C}""",
        z_level=2,
        strip=False,
    )
    t.text_anchor_box(1, bg_color="orange", z_level=1)

    t = slide.text(
        "Hello ~1{world!}",
        z_level=2,
        strip=False,
    )
    t.text_anchor_box(1, bg_color="orange", z_level=1)


@check()
def test_text_anchors_styled_space_prefix(deck):
    deck.update_style("code", TextStyle(size=22))
    deck.set_style("s1", TextStyle(size=2))
    deck.set_style("s2", TextStyle(size=44))
    slide = deck.new_slide(width=200, height=100)
    t = slide.text("""~s1{   }~s2{ }~11{C}""", z_level=2)
    t.line_box(0, bg_color="gray", z_level=0)
    t.text_anchor_box(11, bg_color="orange", z_level=1)
    t = slide.text("""~s1{ }~s2{        }~s1{ }~11{C}""", z_level=2)
    t.line_box(0, bg_color="gray", z_level=0)
    t.text_anchor_box(11, bg_color="orange", z_level=1)


@check(n_slides=3)
def test_text_in_steps(deck):
    slide = deck.new_slide(width=200, height=100)
    slide.text(InSteps(["one", "two", "three"]))


@check(n_slides=3)
def test_array_text_in_steps(deck):
    slide = deck.new_slide(width=100, height=30)
    slide.set_style("default", TextStyle(size=12))
    slide.text(["Hello ", InSteps(["world", "Nelsie", "user"]), " ", InSteps({2: "!"})])
