from testutils import check

from nelsie import TextStyle


@check(n_slides=2)
def test_code_rust_syntax_highlight(deck):
    deck.update_style("code", TextStyle(size=12))
    slide = deck.new_slide(width=300, height=150)
    slide.code(
        """
/*
  This is the main function.
*/
fn main() {
    // Print text to the console.
    println!("Hello World!");
}
""",
        language="rs",
    )
    slide = deck.new_slide(width=300, height=150)
    slide.code(
        """
/*
  This is the main function.
*/
fn main() {
    // Print text to the console.
    println!("Hello World!");
}
""",
        language="rs",
        theme="Solarized (light)",
    )


@check()
def test_code_syntax_highlight_with_styles(deck):
    deck.update_style("code", TextStyle(size=12))
    deck.set_style("s1", TextStyle(size=16, color="green"))
    deck.set_style("s2", TextStyle(size=16, color="orange"))
    slide = deck.new_slide(width=300, height=210)
    slide.code(
        """
/*
  This ~s1{is} the main function.
*/
f~s1{n} ~s1{main}() {
    // Print text to the console.
    p~s1{rintl}n!(~s1{"Hello World!"});
}

~s2{
// test1
// test2
}
""",
        language="rs",
        parse_styles=True,
    )


@check()
def test_code_syntax_highlight_anchors(deck):
    deck.update_style("code", TextStyle(size=12))
    deck.set_style("s1", TextStyle(size=16, color="green"))
    slide = deck.new_slide(width=300, height=210)
    t = slide.code(
        """
# ~11{C}omment

def ~1{main}():
    p~s1{r~100{int}(}~102{~101{"Hello} w~103{o}rld!"})

if ~2{__name__ == "__main__"}:
    main()
""",
        language="py",
        parse_styles=True,
        z_level=2,
    )
    t.text_anchor_box(1, bg_color="orange", z_level=0)
    t.text_anchor_box(2, bg_color="orange", z_level=0)

    t.text_anchor_box(100, bg_color="orange", z_level=1)
    t.text_anchor_box(101, bg_color="red", z_level=1)
    t.text_anchor_box(102, bg_color="green", z_level=0)
    t.text_anchor_box(103, bg_color="blue", z_level=1)

    t.text_anchor_box(10, bg_color="blue", z_level=1)
    t.text_anchor_box(11, bg_color="blue", z_level=1)


@check()
def test_code_style_delimiters(deck):
    slide = deck.new_slide(width=300, height=50)
    slide.set_style("big", TextStyle(size=20, color="orange"))
    slide.code(
        "print('$big<Hello> world!')",
        "Python",
        style=TextStyle(size=12),
        parse_styles=True,
        style_delimiters="$<>",
    )


@check()
def test_code_language_none(deck):
    slide = deck.new_slide(width=200, height=30)
    slide.code("print('Hello world!')", None, style=TextStyle(size=12))


@check(deck_kwargs=dict(default_code_language="Python"))
def test_code_language_default(deck):
    slide = deck.new_slide(width=200, height=30)
    slide.code("print('Hello world!')", style=TextStyle(size=12))
