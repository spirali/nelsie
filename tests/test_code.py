from nelsie import TextStyle
from testutils import check


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
