from nelsie import TextStyle
from testutils import check
from nelsie.textsteps import text_step_parser


def test_text_step_parser():
    r = text_step_parser("abc**2-3", "**")
    assert sorted(r.named_steps) == [2, 3]
    assert r.values == {1: "", 2: "abc", 4: ""}

    r = text_step_parser("abc", "**")
    assert sorted(r.named_steps) == []
    assert r.values == {1: "abc"}

    r = text_step_parser("abc\nxyz", "**")
    assert sorted(r.named_steps) == []
    assert r.values == {1: "abc\nxyz"}

    r = text_step_parser("line1 ** 3+\nline2\nline3 ** 1\nline4**4+", "**")
    assert sorted(r.named_steps) == [1, 3, 4]
    assert r.values == {1: "line2\nline3 ", 2: "line2", 3: "line1 \nline2", 4: "line1 \nline2\nline4"}

    r = text_step_parser("line1 ** e; 2\nline 2", "**")
    assert sorted(r.named_steps) == [2]
    assert r.values == {1: "\nline 2", 2: "line1 \nline 2", 3: "\nline 2"}

    r = text_step_parser("line1 ** n; 2-3", "**")
    assert sorted(r.named_steps) == [2, 3]
    assert r.values == {1: "line1 ", 2: "", 4: "line1 "}

    r = text_step_parser("line1 ** en; 2-3\nx", "**")
    assert sorted(r.named_steps) == [2, 3]
    assert r.values == {1: "line1 \nx", 2: "\nx", 4: "line1 \nx"}


@check(3)
def test_parse_steps_in_and_code_text(deck):
    deck.set_style("default", TextStyle(size=12))

    slide = deck.new_slide(width=250, height=100)
    slide.text("Line 1\nLine **2+", parse_steps=True)
    slide.code(
        """
def do_something(): **1-2
def do_something():  # Tada! **3+
    println("Hello")** 2+    
    """,
        "py",
        parse_steps=True,
        x=10,
    )
