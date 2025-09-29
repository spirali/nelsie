from nelsie import Arrow, Path, SlideDeck, TextStyle, Resources, Stroke, StepVal, Point, Oval, Rect, GridOptions

resources = Resources()
resources.load_fonts_dir("karla_font")
deck = SlideDeck(resources=resources)

COLOR1 = "#333"
COLOR2 = "#ddccdd"

deck.update_style("default", TextStyle(font="Karla", color=COLOR1))
deck.set_style("highlight", TextStyle(color="#cc99cc", weight=700))
deck.set_style("title", TextStyle(size=64))


# First slide #############################################


@deck.slide()
def intro(slide):
    slide.image("../../docs/imgs/nelsie-logo.jpg", width="25%")
    title_box = slide.box(width="100%", p_top=20, p_bottom=30, m_top=50, m_bottom=50, bg_color=COLOR2)
    title_box.text("Nelsie", style=TextStyle(size=64, weight=800), m_bottom=15)
    title_box.text("Framework for Creating Slides", style=TextStyle(size=44))

    slide.set_style("email", TextStyle(font="monospace", size=18))
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
        y=StepVal(20).at(2, 110).at(3, 210),
        width=80,
        height=80,
        bg_color=StepVal("red").at(2, "orange").at(3, "green"),
    )


# Layers in SVGs and ORA images ##########################################


@deck.slide()
def images(slide):
    slide.text("Name-driven revealing layers in SVG and ORA images", m_bottom=40)
    slide.image("assets/stepped_logo.ora", width="25%")
    slide.image("assets/layers.svg", width="80%")


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
    a = text.line_p(3, 1, 0.5)
    b = text.line_p(9, 1, 0.5)
    slide.add(
        Path(stroke=Stroke(color="orange", width=5), arrow_end=arrow)
        .move_to(a.move_by(5, 0))
        .line_to(Point(b.x + 50, a.y))
        .line_to(b.move_by(50, 0))
        .line_to(b.move_by(5, 0))
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

    slide.add(
        Path(fill_color="green")
        .move_to(text.line_p(5, 1.0, 0.5).move_by(5, 0))
        .line_to(comment.p(0.1, 0.25))
        .line_to(comment.p(0.1, 0.85))
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
        text.inline_box(anchor_id, bg_color="#aaa", z_level=-1)

    arrow = Arrow(size=15)
    slide.add(
        Path(stroke=Stroke(color="green", width=5), arrow_end=arrow)
        .move_to(text.inline_p(2, 0, 1).move_by(-20, 20))
        .line_to(text.inline_p(2, 0, 1))
    )
    slide.add(
        Path(stroke=Stroke(color="green", width=5), arrow_end=arrow)
        .move_to(text.inline_p(2, 1, 1).move_by(20, 20))
        .line_to(text.inline_p(2, 1, 1))
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


# Showing/hiding lines #################################


@deck.slide()
def show_hide_line(slide):
    slide.text("Line hiding/showing", "title", m_bottom=30)
    slide.set_style("hl", TextStyle(color="orange", weight=700))

    bb = slide.box(align_items="start")
    top_b = bb.box(bg_color="#eee").padding(x=20, y=10)
    top_b.text("Definition (step definition after **)", m_bottom=20, x=20)

    x = '"""'
    top_b.code(
        f"""
@deck.slide()
def line_demo(slide):
    slide.code({x}
def compute_somehing(x): ~hl[**1]
def compute_somehing(x, y): ~hl[**2+]
    print("Computing...") ~hl[**e; 3+]
    return x * y ~hl[**e; 4+]
{x})
""",
        language="py",
        parse_styles=True,
        style_delimiters="~[]",
        style=TextStyle(size=24),
    )

    bot_b = bb.box(bg_color="#eee", width=top_b.width()).padding(x=20, y=10).margin(top=40)
    bot_b.text("Result", m_bottom=20, x=20)
    demo = """def compute_somehing(x): **1
def compute_somehing(x, y): **2+
    print("Computing...") **e; 3+
    return x * y **e; 4+"""
    bot_b.code(demo, language="py", parse_steps=True, style=TextStyle(size=24))


# Console demo ############################################


@deck.slide()
def console_demo(slide):
    slide.set_style("shell", TextStyle(font="monospace", color="white", size=24))
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
    rect = Rect(Point(0, 0), Point(100, 100), stroke=Stroke(color="red", width=5))
    box.add(rect)
    triangle = Path(fill_color="green").move_to(Point(200, 100)).line_to(Point(250, 0)).line_to(Point(300, 100))
    box.add(triangle)
    circle = Oval(Point(400, 0), Point(500, 100), fill_color="blue")
    box.add(circle)

    rounded_box = (
        Path(stroke=Stroke(color="orange", width=5))
        .move_to(Point(650, 0))
        .quad_to(Point(700, 0), Point(700, 50))
        .quad_to(Point(700, 100), Point(650, 100))
        .quad_to(Point(700, 100), Point(650, 100))
        .quad_to(Point(600, 100), Point(600, 50))
        .line_to(Point(650, 50))
    )
    box.add(rounded_box)

    box = slide.box(width=700, height=70, m_bottom=60)
    box.add(Path(stroke=Stroke(color="black", width=10, dash_array=[10])).move_to(Point(0, 0)).line_to(Point(700, 0)))
    box.add(
        Path(stroke=Stroke(color="black", width=10, dash_array=[10, 20], dash_offset=15))
        .move_to(Point(0, 30))
        .line_to(Point(700, 30))
    )
    box.add(
        Path(stroke=Stroke(color="black", width=10, dash_array=[30, 10, 5, 10]))
        .move_to(Point(0, 60))
        .line_to(Point(700, 60))
    )

    box = slide.box(width=700, height=220)
    arrow1 = Arrow(size=30)
    arrow2 = Arrow(size=30, angle=20)
    arrow3 = Arrow(size=30, inner_point=0.8)
    arrow4 = Arrow(size=30, inner_point=2.4)
    arrow5 = Arrow(size=30, stroke_width=5)

    box.add(
        Path(
            stroke=Stroke(color="black", width=5),
            arrow_start=arrow1,
            arrow_end=arrow1,
        )
        .move_to(Point(0, 0))
        .line_to(Point(700, 0))
    )
    box.add(
        Path(
            stroke=Stroke(color="black", width=5),
            arrow_start=arrow2,
            arrow_end=arrow2,
        )
        .move_to(Point(0, 50))
        .line_to(Point(700, 50))
    )
    box.add(
        Path(
            stroke=Stroke(color="black", width=5),
            arrow_start=arrow3,
            arrow_end=arrow3,
        )
        .move_to(Point(0, 100))
        .line_to(Point(700, 100))
    )
    box.add(
        Path(
            stroke=Stroke(color="black", width=5),
            arrow_start=arrow4,
            arrow_end=arrow4,
        )
        .move_to(Point(0, 150))
        .line_to(Point(700, 150))
    )
    box.add(
        Path(
            stroke=Stroke(color="black", width=5),
            arrow_start=arrow5,
            arrow_end=arrow5,
        )
        .move_to(Point(0, 200))
        .line_to(Point(700, 200))
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
    p0 = root.p(1, 0.5)
    p1 = child1.p(0, 0.5)
    p2 = child2.p(0, 0.5)

    a = child1.p(1, 0.5)
    b = child1.p(0.5, 0)

    slide.add(
        Path(stroke=Stroke(color="#777", width=2), arrow_end=arrow)
        .move_to(p0)
        .cubic_to(p0.move_by(300, 0), p1.move_by(-300, 0), p1)
    )
    slide.add(Path(stroke=Stroke(color="#777", width=2), arrow_end=arrow).move_to(p0).quad_to(p2.move_by(-100, 0), p2))
    slide.add(
        Path(stroke=Stroke(color="#777", width=2), arrow_end=arrow)
        .move_to(a)
        .quad_to(a.move_by(50, 0), a.move_by(50, -50))
        .cubic_to(a.move_by(50, -100), b.move_by(0, -100), b),
    )

    box = slide.box(x=680, y="50%")
    box.add(
        Path(stroke=Stroke(color="#777", width=2), fill_color="orange")
        .move_to(Point(40, 0))
        .line_to(Point(80, 40))
        .line_to(Point(40, 80))
        .line_to(Point(0, 40))
        .close()
    )


# Grid layout ###########################


@deck.slide()
def grid_demo(slide):
    slide.text("Table demo", "title", m_bottom=20)

    data = [
        ("Name", "Time", "Type"),
        ("Jane", 3.5, "A1"),
        ("John", 4.1, "B7"),
        ("Johanna", 12.0, "C1"),
        ("Elise", 12.5, "D4"),
        ("Max", 320.2, "E1"),
    ]

    # Draw the table
    table = slide.box(
        width="70%",
        grid=GridOptions(template_columns=["2fr", "1fr", 130], template_rows=[50] + [40] * (len(data) - 1)),
        bg_color="#ddd",
    )
    header_style = TextStyle(weight=800)
    table.box(grid=GridOptions(column=(1, 4), row=1), bg_color="#fbc")
    for i in range(2, len(data) + 1, 2):
        table.box(grid=GridOptions(column=(1, 4), row=i), bg_color="#eee")
    column1 = table.box(grid=GridOptions(column=2, row=(1, len(data) + 1)))
    stroke = Stroke(color="#888", width=2)
    column1.add(Path(stroke=stroke).move_to(Point(0, 0)).line_to(Point(0, "100%")))
    column1.add(Path(stroke=stroke).move_to(Point("100%", 0)).line_to(Point("100%", "100%")))

    # Fill the table with data
    for i, row in enumerate(data, 1):
        s = header_style if i == 1 else None
        table.box(grid=GridOptions(column=1, row=i)).text(row[0], s)
        table.box(grid=GridOptions(column=2, row=i), row=True, justify_content="end", m_right=30).text(str(row[1]), s)
        table.box(grid=GridOptions(column=3, row=i), row=True, justify_content="start", m_left=30).text(row[2], s)


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
    slide.overlay(show="1-3", z_level=1).add(
        Path(stroke=Stroke(color="black", width=15), arrow_end=Arrow(30))
        .move_to(tiles[(3, 4)].p(0.5, 0.5))
        .line_to(tiles[(3, 2)].p(0.5, 0.5))
        .line_to(tiles[(4, 2)].p(0.5, 0.5))
    )

    # Draw knight
    tiles[(3, 4)].box(show="1", z_level=2).image("assets/knight.svg")
    tiles[(3, 3)].box(show="2", z_level=2).image("assets/knight.svg")
    tiles[(3, 2)].box(show="3", z_level=2).image("assets/knight.svg")
    tiles[(4, 2)].box(show="4", z_level=2).image("assets/knight.svg")


# Links


@deck.slide()
def links(slide):
    slide.text(
        "Clickable links in PDF",
        TextStyle(size=80, underline=True),
        url="https://github.com/spirali/nelsie",
    )


# Video


# @deck.slide()
# def video(slide):
#     slide.text("Embedded video (click to play*)", "title")
#     row = slide.box(row=True, m_top=30)
#     box1 = row.box()
#     box1.video("assets/video.mp4", cover_image="../../docs/imgs/nelsie-logo.jpg", width=450, height=400)
#     box1.text("Without controls")
#     box2 = row.box()
#     box2.video(
#         "assets/video.mp4", cover_image="../../docs/imgs/nelsie-logo.jpg", width=450, height=400, show_controls=True
#     )
#     box2.text("With controls")
#     slide.text("* This functionally depends on pdf viewer", TextStyle(color="gray"), m_top=30)


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

    slide.set_style("email", TextStyle(font="monospace", size=18))
    slide.text("Ada Böhm\n~email{ada@kreatrix.org}", m_bottom=50, align="center")

    slide.text(x="65%", y="20%", text="Debugging\nframes", style="title")


# FINAL RENDER

deck.render("bigdemo.pdf")
