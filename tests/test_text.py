from dataclasses import asdict

import pytest

from conftest import new_resources
from testutils import check

import nelsie
from nelsie import FontStretch, TextStyle, StepVal


def test_text_update():
    s1 = TextStyle(color="green", size=123)
    s2 = TextStyle(size=321, line_spacing=1.5)
    s3 = s1.merge(s2)
    assert s3.color == "green"
    assert s3.size == 321
    assert s3.line_spacing == 1.5


def test_text_invalid_font(deck):
    s1 = TextStyle(font="Nonexisting font")
    deck.new_slide().text("Hello", style=s1)
    with pytest.raises(Exception, match="Font 'Nonexisting font' not found"):
        deck.render("out.pdf")


@check()
def test_text_no_style(deck):
    s = deck.new_slide(width=120, height=40)
    s.text("Hello")


@check()
def test_text_shared(deck):
    s = deck.new_slide(width=100, height=50)
    s.text_style = TextStyle(size=12)
    s.text("Shared")
    s.text("Non-Shared")
    s.text("Shared")


@check()
def test_text_squeeze(deck):
    s = deck.new_slide(width=40, height=35)
    s.text_style = TextStyle(size=12)
    s.text("Shared", width=25)
    s.text("Non-Shared", width=25)
    s.text("Shared", width=25)


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
        StepVal(TextStyle(color="green")).at(2, TextStyle(color="orange", size=64)),
    )
    slide.box().text("Say ~my_style{hello}!")


@check()
def test_render_text_unicode(deck):
    deck.set_style("x", TextStyle(color="yellow"))
    slide = deck.new_slide()
    slide.box(bg_color="#f88").text("Příliš žluťoučký ~x{kůň} úpěl ďábelské ódy\n>>>y̆<<<")


def test_set_get_styles_deck(deck):
    s = deck.get_style("default")
    for key, value in asdict(s).items():
        assert value is not None

    assert deck.get_style("big") is None

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
    slide.text("Test text", TextStyle(line_through=True), m_bottom=10)
    slide.text("Test text", TextStyle(underline=True, line_through=True))


@check()
def test_text_color_opacity(deck):
    slide = deck.new_slide(width=220, height=50)
    slide.set_style("one", TextStyle(color="#ff00ff50"))
    slide.box(x=0, y=0, width=60, height="100%", bg_color="green")
    slide.box(x=60, y=0, width="30%", height="100%", bg_color="blue")
    slide.text("Opacity test", style="one")


@check()
def test_text_styling(deck):
    slide = deck.new_slide(width=220, height=150)
    slide.text("Italic", style=TextStyle(italic=True))
    slide.text("Bold", TextStyle(weight=700))
    slide.text("Hi are you?", TextStyle(stretch=FontStretch.UltraCondensed, size=10))
    slide.text("Hi are you?", TextStyle(stretch=FontStretch.UltraExpanded, size=10))


@check()
def test_text_monospace(deck):
    slide = deck.new_slide(width=150, height=100)
    slide.text("WWW\n111", TextStyle(font="monospace"))


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
    slide = deck.new_slide(width=200, height=325)
    slide.set_style("default", TextStyle(size=24))
    slide.set_style("monospace", TextStyle(font="monospace"))
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
            x=t.inline_x(1),
            y=t.inline_y(1),
            width=t.inline_width(1),
            height=t.inline_height(1),
            bg_color="orange",
        )
        slide.box(
            x=t.inline_x(2),
            y=t.inline_y(2),
            width=t.inline_width(2),
            height=t.inline_height(2),
            bg_color="gray",
        )
        slide.box(
            x=t.inline_x(100),
            y=t.inline_y(100),
            width=t.inline_width(100),
            height=t.inline_height(100),
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
    t.inline_box(123, bg_color="orange", z_level=0)


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
    t.inline_box(1, bg_color="orange", z_level=1)

    t = slide.text(
        "Hello ~1{world!}",
        z_level=2,
        strip=False,
    )
    t.inline_box(1, bg_color="orange", z_level=1)


@check()
def test_text_anchors_styled_space_prefix(deck):
    deck.update_style("code", TextStyle(size=22))
    deck.set_style("s1", TextStyle(size=2))
    deck.set_style("s2", TextStyle(size=44))
    slide = deck.new_slide(width=200, height=120)
    t = slide.text("""~s1{   }~s2{ }~11{C}""", z_level=3)
    t.line_box(0, bg_color="blue", z_level=0)
    t.inline_box(11, bg_color="orange", z_level=1)
    t = slide.text("""~s1{ }~s2{        }~s1{ }~11{C}""", z_level=2)
    t.line_box(0, bg_color="gray", z_level=0)
    t.inline_box(11, bg_color="orange", z_level=1)


@check(n_slides=3)
def test_text_in_steps(deck):
    slide = deck.new_slide(width=200, height=100)
    slide.text(StepVal("one").at(2, "two").at(3, "three"))


def test_set_generic_families():
    res = new_resources()
    res.set_monospace("DejaVu Sans")
    res.set_sans_serif("DejaVu Sans")
    res.set_serif("DejaVu Sans")
    with pytest.raises(Exception, match="Font 'xxx' not found"):
        res.set_serif("xxx")
    with pytest.raises(Exception, match="Font 'xxx' not found"):
        res.set_sans_serif("xxx")
    with pytest.raises(Exception, match="Font 'xxx' not found"):
        res.set_monospace("xxx")
