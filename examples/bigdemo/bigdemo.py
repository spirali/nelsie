from nelsie import (
    Arrow,
    Path,
    SlideDeck,
    TextStyle,
    Resources,
    TextAlign,
    Align,
    Stroke,
    InSteps,
)

resources = Resources()
resources.load_fonts_dir("karla_font")
deck = SlideDeck(resources=resources)

COLOR1 = "black"
COLOR2 = "#ddccdd"

deck.update_style("default", TextStyle(font_family="Karla", color=COLOR1))
deck.set_style("highlight", TextStyle(color="#cc99cc", italic=True, size=46))


# First slide #############################################


@deck.slide()
def intro(slide):
    slide.image("../../docs/imgs/nelsie-logo.jpg", width="25%")
    title_box = slide.box(
        width="100%", p_top=20, p_bottom=30, m_top=50, m_bottom=50, bg_color=COLOR2
    )
    title_box.text("Nelsie", style=TextStyle(size=64, weight=800), m_bottom=15)
    title_box.text("Framework for Creating Slides", style=TextStyle(size=44))

    slide.set_style("email", slide.get_style("monospace").merge(TextStyle(size=18)))
    slide.text(
        "Ada Böhm\n~email{ada@kreatrix.org}", m_bottom=50, align=TextAlign.Center
    )


# Hello world #############################################


@deck.slide()
def hello_world(slide):
    slide.text("~highlight{Hello world} example", m_bottom=40, style=TextStyle(size=42))
    box = slide.box(p_left=20, p_right=20, p_top=20, p_bottom=20, bg_color="#eee")
    box.code(
        """
from nelsie import SlideDeck

deck = SlideDeck()

@deck.slide()
def hello_world(slide):
    slide.text("Hello world!")

deck.render(output_pdf="slides.pdf")
""",
        language="py",
    )


# Fragments ##########################################


@deck.slide()
def fragments(slide):
    slide.text("Nelsie supports ...")

    # A single slide may generate more pages in resulting pdf
    # Argument 'show' controls on which pages of the slides is the box shown.
    # Possible values:
    # "2" - Show only on the second page
    # "2+" - Show from the second page to the last page
    # "2-5" - Show on second page to 5th page
    # "2,3,4" - Show on pages 2, 3, and 4
    slide.box(show="2+").text("... fragment ...")
    slide.box(show="3+").text("... revealing.")


# InSteps ##########################################


@deck.slide()
def in_steps(slide):
    slide.text("Simple mechanism for complex slide changes", m_bottom=40)

    semaphore = slide.box(width=100, height=300, bg_color="gray")
    semaphore.box(
        y=InSteps({1: 10, 2: 110, 3: 210}),
        width=80,
        height=80,
        bg_color=InSteps({1: "red", 2: "orange", 3: "green"}),
    )


# Layers in SVGs and ORA images ##########################################


@deck.slide()
def images(slide):
    slide.text("Name-driven revealing layers in SVG and ORA images", m_bottom=40)
    slide.image("imgs/stepped_logo.ora", width="25%")
    slide.image("imgs/layers.svg", width="80%")


# Header ##########################################


@deck.slide()
def header_and_footer(slide):
    header = slide.box(
        row=True,
        width="100%",
        bg_color="#aa99aa",
        justify_content=Align.End,
        p_top=25,
        p_bottom=25,
        p_right=20,
    )
    header.text("Header Demo", TextStyle(color="white"))
    slide.box(width="100%", height="10", bg_color="gray")

    slide.box(flex_grow=1).text("Content", TextStyle(size=48))

    footer = slide.box(row=True, width="100%", height="50", bg_color=COLOR2)
    footer.box(flex_grow=1).text("Hello!", TextStyle(size=24, color="gray"))
    footer.box(flex_grow=1, height="100%", bg_color="#aa99aa").text(
        "Footer", TextStyle(size=24)
    )
    footer.box(flex_grow=1).text("Hello!", TextStyle(size=24, color="gray"))


# Syntax highlighting #################################

CODE_EXAMPLE = """
# This is comment
from nelsie import SlideDeck

deck = SlideDeck()

@deck.slide()
def hello_world(slide):
    slide.text("Hello world!")

deck.render(output_pdf="slides.pdf")
"""


@deck.slide()
def text_styles(slide):
    slide.text("Syntax highlighting", TextStyle(size=64), m_bottom=50)

    slide.code(CODE_EXAMPLE, language="py")


# Line highlighting #################################


@deck.slide()
def text_styles(slide):
    slide.text("Line highlighting", TextStyle(size=64), m_bottom=50)

    text = slide.code(CODE_EXAMPLE, language="py")

    text.line_box(6, bg_color="#bb99bb", z_level=-1)


# Pointing into text (1) #################################


@deck.slide()
def text_styles(slide):
    slide.text("Pointing into text", TextStyle(size=64), m_bottom=50)

    text = slide.code(CODE_EXAMPLE, language="py")

    arrow = Arrow(size=20)
    slide.draw(
        Path(stroke=Stroke(color="orange", width=5), arrow_end=arrow)
        .move_to(text.line_x(3, 1) + 5, text.line_y(3, 0.5))
        .line_to(text.line_x(9, 1) + 50, text.line_y(3, 0.5))
        .line_to(text.line_x(9, 1) + 50, text.line_y(9, 0.5))
        .line_to(text.line_x(9, 1) + 5, text.line_y(9, 0.5))
    )


# Pointing into text (2) #################################


@deck.slide()
def text_styles(slide):
    slide.text("Pointing into text", TextStyle(size=64), m_bottom=50)

    text = slide.code(CODE_EXAMPLE, language="py")

    comment = slide.box(
        x="70%",
        y="45%",
        p_left=30,
        p_right=30,
        p_top=30,
        p_bottom=30,
        bg_color="green",
        border_radius=8,
    )
    comment.text("Comment", style=TextStyle(color="white"))

    slide.draw(
        Path(fill_color="green")
        .move_to(text.line_x(5, 1.0) + 5, text.line_y(5, 0.5))
        .line_to(comment.x(0.1), comment.y(0.25))
        .line_to(comment.x(0.1), comment.y(0.75))
    )


# Pointing into text (3) #################################


@deck.slide()
def text_styles(slide):
    slide.text("Pointing into text", TextStyle(size=64), m_bottom=50)

    text = slide.code(
        """
# This is comment
from nelsie import SlideDeck

deck = SlideDeck()

@deck.slide()
def hello_world(~1{slide}):
    ~2{slide}.text("Hello world!")

deck.render(output_pdf="slides.pdf")
""",
        language="py",
        parse_styles=True,
    )

    for anchor_id in 1, 2:
        text.text_anchor_box(anchor_id, bg_color="#aaa", z_level=-1)

    arrow = Arrow(size=15)
    slide.draw(
        [
            Path(stroke=Stroke(color="green", width=5), arrow_end=arrow)
            .move_to(text.text_anchor_x(2, 0) - 20, text.text_anchor_y(2, 1) + 20)
            .line_to(text.text_anchor_x(2, 0), text.text_anchor_y(2, 1)),
            Path(stroke=Stroke(color="green", width=5), arrow_end=arrow)
            .move_to(text.text_anchor_x(2, 1) + 20, text.text_anchor_y(2, 1) + 20)
            .line_to(text.text_anchor_x(2, 1), text.text_anchor_y(2, 1)),
        ]
    )


# Overwriting styles in syntax highlight #################################


@deck.slide()
def text_styles(slide):
    slide.text("Own styles in syntax highlight", TextStyle(size=64), m_bottom=50)

    slide.set_style("grayout", TextStyle(color="gray", weight=400))
    text = slide.code(
        """
~grayout{# This is comment
from nelsie import SlideDeck

deck = SlideDeck()}

@deck.slide()
def hello_world(~1{slide}):
    ~2{slide}.text("Hello world!")

~grayout{deck.render(output_pdf="slides.pdf")}
""",
        language="py",
        parse_styles=True,
    )


# Fragments ##########################################


@deck.slide()
def text_styles(slide):
    box = slide.box(gap=(0, 40))
    box.text("Text size", TextStyle(size=64))

    box.set_style("r", TextStyle(color="red"))
    box.set_style("g", TextStyle(color="green"))
    box.set_style("b", TextStyle(color="blue"))
    box.set_style("bold", TextStyle(weight=700))
    box.set_style("it", TextStyle(italic=True))

    box.text("~bold{bold} ~monospace{monospace} ~it{italics}")
    box.text("~r{red} ~g{green} ~b{blue}")
    box.text("Stroke & fill", TextStyle(stroke=Stroke(color=COLOR1), color="green"))
    box.text(
        "Stroke without fill", TextStyle(stroke=Stroke(color=COLOR1), color="empty")
    )


# FINAL RENDER

deck.render(output_pdf="bigdemo.pdf")
