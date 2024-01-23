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
