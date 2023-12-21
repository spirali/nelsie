from nelsie import SlideDeck, TextStyle, Resources, TextAlign, Align, Stroke, InSteps

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
    #     box = slide.box(p_left=20, p_right=20, p_top=20, p_bottom=20, bg_color="#eee")
    #     box.code(
    #         """
    # semaphore = slide.box(width=100, height=300, bg_color="gray")
    # semaphore.box(
    #     y=InSteps({1: 10, 2: 110, 3: 210}),
    #     width=80,
    #     height=80,
    #     bg_color=InSteps({1: "red", 2: "orange", 3: "green"}),
    # )
    # """,
    #         language="py",
    #         style=TextStyle(size=20)
    #     )

    semaphore = slide.box(width=100, height=300, bg_color="gray")
    semaphore.box(
        y=InSteps({1: 10, 2: 110, 3: 210}),
        width=80,
        height=80,
        bg_color=InSteps({1: "red", 2: "orange", 3: "green"}),
    )


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
