from nelsie import (
    Arrow,
    Path,
    SlideDeck,
    TextStyle,
    Resources,
    Stroke,
    InSteps,
)

resources = Resources()
resources.load_fonts_dir("karla_font")
deck = SlideDeck(resources=resources)

COLOR1 = "#333"
COLOR2 = "#ddccdd"

deck.update_style("default", TextStyle(font_family="Karla", color=COLOR1))
deck.set_style("highlight", TextStyle(color="#cc99cc", weight=800))
deck.set_style("title", TextStyle(size=64))


# First slide #############################################


@deck.slide()
def intro(slide):
    slide.image("../../docs/imgs/nelsie-logo.jpg", width="25%")
    title_box = slide.box(width="100%", p_top=20, p_bottom=30, m_top=50, m_bottom=50, bg_color=COLOR2)
    title_box.text("Nelsie", style=TextStyle(size=64, weight=800), m_bottom=15)
    title_box.text("Framework for Creating Slides", style=TextStyle(size=44))

    slide.set_style("email", slide.get_style("monospace").merge(TextStyle(size=18)))
    slide.text("Ada Böhm\n~email{ada@kreatrix.org}", m_bottom=50, align="center")


# Hello world #############################################


@deck.slide()
def hello_world(slide):
    slide.text("Hello world example", style="title", m_bottom=40)
    box = slide.box(p_left=20, p_right=20, p_top=20, p_bottom=20, bg_color="#eee")
    box.code(
        """
from nelsie import SlideDeck

deck = SlideDeck()

@deck.slide()
def hello_world(slide):
    slide.text("Hello world!")

deck.render("slides.pdf")
""",
        language="py",
    )


# Fragments ##########################################


@deck.slide()
def fragments(slide):
    slide.text("Nelsie supports ...", style="title")

    # A single slide may generate more pages in resulting pdf
    # Argument 'show' controls on which pages of the slides is the box shown.
    # Possible values:
    # "2" - Show only on the second page
    # "2+" - Show from the second page to the last page
    # "2-5" - Show on second page to 5th page
    # "2,3,4" - Show on pages 2, 3, and 4
    slide.box(show="2+").text("... fragment ...", style="title")
    slide.box(show="3+").text("... revealing.", style="title")


# InSteps ##########################################


@deck.slide()
def in_steps(slide):
    slide.text("Simple mechanism for complex slide changes", m_bottom=40)

    semaphore = slide.box(width=100, height=300, bg_color="gray")
    semaphore.box(
        y=InSteps({1: 20, 2: 110, 3: 210}),
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
        justify_content="end",
        p_top=25,
        p_bottom=25,
        p_right=20,
    )
    header.text("Header Demo", TextStyle(color="white"))
    slide.box(width="100%", height="10", bg_color="gray")

    slide.box(flex_grow=1).text("Headers & Titles", style="title")

    footer = slide.box(row=True, width="100%", height="50", bg_color=COLOR2)
    footer.box(flex_grow=1).text("Hello!", TextStyle(size=24, color="gray"))
    footer.box(flex_grow=1, height="100%", bg_color="#aa99aa").text("Footer", TextStyle(size=24))
    footer.box(flex_grow=1).text("Hello!", TextStyle(size=24, color="gray"))


# Syntax highlighting #################################

CODE_EXAMPLE = """
# This is comment
from nelsie import SlideDeck

deck = SlideDeck()

@deck.slide()
def hello_world(slide):
    slide.text("Hello world!")

deck.render("slides.pdf")
"""


@deck.slide()
def syntax_highlighting(slide):
    slide.text("Syntax highlighting", "title", m_bottom=50)

    slide.code(CODE_EXAMPLE, language="py")


# Line highlighting #################################


@deck.slide()
def line_highlighting(slide):
    slide.text("Line highlighting", "title", m_bottom=50)

    text = slide.code(CODE_EXAMPLE, language="py")

    text.line_box(6, bg_color="#bb99bb", z_level=-1)


# Pointing into text (1) #################################


@deck.slide()
def text_pointers1(slide):
    slide.text("Pointing into text", "title", m_bottom=50)

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
def text_pointers2(slide):
    slide.text("Pointing into text", "title", m_bottom=50)

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
def text_pointers3(slide):
    slide.text("Pointing into text", "title", m_bottom=50)

    text = slide.code(
        """
# This is comment
from nelsie import SlideDeck

deck = SlideDeck()

@deck.slide()
def hello_world(~1{slide}):
    ~2{slide}.text("Hello world!")

deck.render("slides.pdf")
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
def syntax_highlight_with_own_styles(slide):
    slide.text("Own styles in syntax highlight", "title", m_bottom=50)

    slide.set_style("grayout", TextStyle(color="gray", weight=400))
    slide.code(
        """
~grayout{# This is comment
from nelsie import SlideDeck

deck = SlideDeck()}

@deck.slide()
def hello_world(~1{slide}):
    ~2{slide}.text("Hello world!")

~grayout{deck.render("slides.pdf")}
""",
        language="py",
        parse_styles=True,
    )


# Console demo ############################################


@deck.slide()
def console_demo(slide):
    slide.set_style("shell", slide.get_style("code").merge(TextStyle(color="white", size=24)))
    slide.set_style("prompt", TextStyle(color="#aaaaff"))
    slide.set_style("cmd", TextStyle(color="yellow"))

    slide.text("Console demo", "title", m_bottom=30)

    # The 'console' is just text with a few styles
    # As we want to use "~" character in the text,
    # we are changing escape_char for styles from "~" to "!"
    console = slide.box(bg_color="black", p_left=20, p_right=20, p_top=10, p_bottom=10, border_radius=10)
    console.text(
        "!prompt{~/nelsie/example/bigdemo$} !cmd{ls}\n"
        "bigdemo.py  imgs  karla_font\n\n"
        "!prompt{~/nelsie/example/bigdemo$} !cmd{python3 bigdemo.py}\n",
        style="shell",
        style_delimiters="!{}",
    )


# Shapes ##########################################


@deck.slide()
def shapes(slide):
    slide.text("Shapes", "title", m_bottom=40)

    box = slide.box(width=700, height=100, m_bottom=60)
    rect = (
        Path(stroke=Stroke(color="red", width=5))
        .move_to(0, 0)
        .line_to(100, 0)
        .line_to(100, 100)
        .line_to(0, 100)
        .close()
    )
    triangle = Path(fill_color="green").move_to(200, 100).line_to(250, 0).line_to(300, 100)
    circle = Path.oval(400, 0, 500, 100, fill_color="blue")

    rounded_box = (
        Path(stroke=Stroke(color="orange", width=5))
        .move_to(650, 0)
        .quad_to(700, 0, 700, 50)
        .quad_to(700, 100, 650, 100)
        .quad_to(700, 100, 650, 100)
        .quad_to(600, 100, 600, 50)
        .line_to(650, 50)
    )
    box.draw([rect, triangle, rounded_box, circle])

    box = slide.box(width=700, height=70, m_bottom=60)
    box.draw(
        [
            Path(stroke=Stroke(color="black", width=10, dash_array=[10])).move_to(0, 0).line_to(700, 0),
            Path(stroke=Stroke(color="black", width=10, dash_array=[10, 20], dash_offset=15))
            .move_to(0, 30)
            .line_to(700, 30),
            Path(stroke=Stroke(color="black", width=10, dash_array=[30, 10, 5, 10])).move_to(0, 60).line_to(700, 60),
        ]
    )

    box = slide.box(width=700, height=220)
    arrow1 = Arrow(size=30)
    arrow2 = Arrow(size=30, angle=20)
    arrow3 = Arrow(size=30, inner_point=0.8)
    arrow4 = Arrow(size=30, inner_point=2.4)
    arrow5 = Arrow(size=30, stroke_width=5)

    box.draw(
        [
            Path(
                stroke=Stroke(color="black", width=5),
                arrow_start=arrow1,
                arrow_end=arrow1,
            )
            .move_to(0, 0)
            .line_to(700, 0),
            Path(
                stroke=Stroke(color="black", width=5),
                arrow_start=arrow2,
                arrow_end=arrow2,
            )
            .move_to(0, 50)
            .line_to(700, 50),
            Path(
                stroke=Stroke(color="black", width=5),
                arrow_start=arrow3,
                arrow_end=arrow3,
            )
            .move_to(0, 100)
            .line_to(700, 100),
            Path(
                stroke=Stroke(color="black", width=5),
                arrow_start=arrow4,
                arrow_end=arrow4,
            )
            .move_to(0, 150)
            .line_to(700, 150),
            Path(
                stroke=Stroke(color="black", width=5),
                arrow_start=arrow5,
                arrow_end=arrow5,
            )
            .move_to(0, 200)
            .line_to(700, 200),
        ]
    )


# Path demo ##########################################


@deck.slide()
def path_demo(slide):
    root = slide.box(
        x=250,
        y="50%",
        bg_color="#777",
        border_radius=10,
        p_left=20,
        p_right=20,
        p_top=20,
        p_bottom=20,
    )
    root.text("Root", style=TextStyle(color="white"))
    child1 = slide.box(
        x=650,
        y="20%",
        bg_color="#777",
        border_radius=10,
        p_left=20,
        p_right=20,
        p_top=20,
        p_bottom=20,
    )
    child1.text("Child 1", style=TextStyle(color="white"))
    child2 = slide.box(
        x=650,
        y="80%",
        bg_color="#777",
        border_radius=10,
        p_left=20,
        p_right=20,
        p_top=20,
        p_bottom=20,
    )
    child2.text("Child 2", style=TextStyle(color="white"))
    arrow = Arrow(20)
    x0, y0 = root.x(1), root.y(0.5)
    x1, y1 = child1.x(0), child1.y(0.5)
    x2, y2 = child2.x(0), child2.y(0.5)

    x1a, y1a = child1.x(1), child1.y(0.5)
    x1b, y1b = child1.x(0.5), child1.y(0)

    slide.draw(
        [
            Path(stroke=Stroke(color="#777", width=2), arrow_end=arrow)
            .move_to(x0, y0)
            .cubic_to(x0 + 300, y0, x1 - 300, y1, x1, y1),
            Path(stroke=Stroke(color="#777", width=2), arrow_end=arrow).move_to(x0, y0).quad_to(x2 - 100, y2, x2, y2),
            Path(stroke=Stroke(color="#777", width=2), arrow_end=arrow)
            .move_to(x1a, y1a)
            .quad_to(x1a + 50, y1a, x1a + 50, y1a - 50)
            .cubic_to(x1a + 50, y1a - 100, x1b, y1b - 100, x1b, y1b),
        ]
    )

    box = slide.box(x=680, y="50%")
    box.draw(
        Path(stroke=Stroke(color="#777", width=2), fill_color="orange")
        .move_to(40, 0)
        .line_to(80, 40)
        .line_to(40, 80)
        .line_to(0, 40)
        .close()
    )


# Chessboard demo ##########################################


@deck.slide()
def chess_board(slide):
    # Draw chessboard
    colors = [COLOR2, "#665566"]
    tiles = {}
    board = slide.box(width=500, height=500)
    for i in range(8):
        row = board.box(width="100%", flex_grow=1.0, row=True)
        for j in range(8):
            b = row.box(height="100%", flex_grow=1.0, bg_color=colors[(i + j) % 2])
            tiles[(j, i)] = b.overlay(z_level=0)

    # Draw arrow
    slide.overlay(show="1-3", z_level=1).draw(
        Path(stroke=Stroke(color="black", width=15), arrow_end=Arrow(30))
        .move_to(tiles[(3, 4)].x(0.5), tiles[(3, 4)].y(0.5))
        .line_to(tiles[(3, 2)].x(0.5), tiles[(3, 2)].y(0.5))
        .line_to(tiles[(4, 2)].x(0.5), tiles[(4, 2)].y(0.5))
    )

    # Draw knight
    tiles[(3, 4)].box(show="1", z_level=2).image("imgs/knight.svg")
    tiles[(3, 3)].box(show="2", z_level=2).image("imgs/knight.svg")
    tiles[(3, 2)].box(show="3", z_level=2).image("imgs/knight.svg")
    tiles[(4, 2)].box(show="4", z_level=2).image("imgs/knight.svg")


# Links


@deck.slide()
def links(slide):
    slide.text("Clicable links in PDF", TextStyle(size=80, underline=True), url="https://github.com/spirali/nelsie")


# Debugging frames ##########################################


@deck.slide(debug_layout=True)
def debugging_frames(slide):
    slide.image("../../docs/imgs/nelsie-logo.jpg", width="25%")
    title_box = slide.box(
        width="100%",
        p_top=20,
        p_bottom=30,
        m_top=50,
        m_bottom=50,
        bg_color=COLOR2,
        name="title box",
        debug_layout="black",
    )
    title_box.text("Nelsie", style=TextStyle(size=64, weight=800), m_bottom=15)
    title_box.text("Framework for Creating Slides", style=TextStyle(size=44))

    slide.set_style("email", slide.get_style("monospace").merge(TextStyle(size=18)))
    slide.text("Ada Böhm\n~email{ada@kreatrix.org}", m_bottom=50, align="center")

    slide.text(x="65%", y="20%", text="Debugging\nframes", style="title")


# FINAL RENDER

deck.render("bigdemo.pdf")
