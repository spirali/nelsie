from nelsie import Stroke, Path, TextStyle, GridOptions as G, Point
from testutils import check


@check()
def test_fix_sizes(deck):
    slide = deck.new_slide()
    slide.box(width="100", height=300, bg_color="red")
    slide.box(width="100%", height="20", bg_color="blue")
    slide.box(width="50%", height="20%", bg_color="green")


@check(n_slides=4)
def test_layout_directions(deck):
    def make_slide(box):
        box.box(width=100, height=100, bg_color="red")
        box.box(width=200, height=200, bg_color="green")
        box.box(width=300, height=300, bg_color="blue")

    make_slide(deck.new_slide().box(width="100%", height="100%", row=False, reverse=False))
    make_slide(deck.new_slide().box(width="100%", height="100%", row=False, reverse=True))
    make_slide(deck.new_slide().box(width="100%", height="100%", row=True, reverse=False))
    make_slide(deck.new_slide().box(width="100%", height="100%", row=True, reverse=True))


@check(n_slides=5)
def test_layout_position(deck):
    def make_test_slide(row=False, **kwargs):
        slide = deck.new_slide(width=400, height=300)
        b = slide.box(row=row, width="50%", height="50%", bg_color="gray")
        b.box(width=20, height=20, bg_color="green")
        b2 = b.box(width=20, height=20, bg_color="red", **kwargs)
        b2.box(width="70%", height="70%", bg_color="orange")
        b.box(width=20, height=20, bg_color="green")

    # Slide 0 - both position defined
    make_test_slide(x=40, y=130)

    # Slide 1 - only x defined
    make_test_slide(x=40)

    # Slide 2 - only y defined
    make_test_slide(y=130)

    # Slide 3 - only x defined (row)
    make_test_slide(x=40, row=True)

    # Slide 4 - only y defined (row)
    make_test_slide(y=130, row=True)


@check()
def test_layout_position_string(deck):
    slide = deck.new_slide(width=400, height=300)
    slide.box(x="30", y="30%", width="50", height="50", bg_color="red")
    slide.box(x=30, y="80%", width="50", height="50", bg_color="blue")


@check()
def test_z_level(deck):
    slide = deck.new_slide(width=100, height=100)
    slide.box(x=0, y=0, width=75, height=75, z_level=1, bg_color="red")
    slide.box(x=25, y=25, width=75, height=75, bg_color="green")


@check()
def test_layout_padding(deck):
    slide = deck.new_slide(width=150, height=150)
    b = slide.box(bg_color="gray", p_left=20)
    b.box(width=15, height=15, bg_color="red")
    b = slide.box(bg_color="gray", p_right=20)
    b.box(width=15, height=15, bg_color="blue")
    b = slide.box(bg_color="gray", p_top=20)
    b.box(width=15, height=15, bg_color="red")
    b = slide.box(bg_color="gray", p_bottom=20)
    b.box(width=15, height=15, bg_color="blue")


@check()
def test_layout_margin(deck):
    slide = deck.new_slide(width=150, height=150)
    b = slide.box(bg_color="gray", m_left=20)
    b.box(width=15, height=15, bg_color="red")
    b = slide.box(bg_color="gray", m_right=20)
    b.box(width=15, height=15, bg_color="blue")
    b = slide.box(bg_color="gray", m_top=20)
    b.box(width=15, height=15, bg_color="red")
    b = slide.box(bg_color="gray", m_bottom=20)
    b.box(width=15, height=15, bg_color="blue")


@check()
def test_layout_flex_grow(deck):
    slide = deck.new_slide(width=150, height=150)
    b = slide.box(height="100%", bg_color="gray")
    b.box(width=30, height=15, bg_color="orange")
    b.box(width=30, flex_grow=1, bg_color="red")
    b.box(width=30, flex_grow=2, bg_color="blue")
    b.box(width=30, height=5, bg_color="#ff00ff")


@check(n_slides=4)
def test_layout_justify_content(deck):
    slide = deck.new_slide(width=50, height=150)
    b = slide.box(height="100%", width="100%", bg_color="gray", justify_content="start")
    b.box(width=20, height=30, bg_color="red")
    b.box(width=20, height=30, bg_color="green")
    b.box(width=20, height=30, bg_color="blue")

    slide = deck.new_slide(width=50, height=150)
    b = slide.box(height="100%", width="100%", bg_color="gray", justify_content="end")
    b.box(width=20, height=30, bg_color="red")
    b.box(width=20, height=30, bg_color="green")
    b.box(width=20, height=30, bg_color="blue")

    slide = deck.new_slide(width=50, height=150)
    b = slide.box(
        height="100%",
        width="100%",
        bg_color="gray",
        justify_content="space-evenly",
    )
    b.box(width=20, height=30, bg_color="red")
    b.box(width=20, height=30, bg_color="green")
    b.box(width=20, height=30, bg_color="blue")

    slide = deck.new_slide(width=50, height=150)
    b = slide.box(
        height="100%",
        width="100%",
        bg_color="gray",
        justify_content="space-around",
    )
    b.box(width=20, height=30, bg_color="red")
    b.box(width=20, height=30, bg_color="green")
    b.box(width=20, height=30, bg_color="blue")


@check(n_slides=3)
def test_layout_gap(deck):
    slide = deck.new_slide(width=50, height=150)
    b = slide.box(
        height="100%",
        width="100%",
        bg_color="gray",
        justify_content="start",
        gap_y=10,
    )
    b.box(width=20, height=30, bg_color="red")
    b.box(width=20, height=30, bg_color="green")
    b.box(width=20, height=30, bg_color="blue")

    slide = deck.new_slide(width=50, height=150)
    b = slide.box(
        height="100%",
        width="100%",
        bg_color="gray",
        justify_content="start",
        gap_y="15%",
    )
    b.box(width=20, height=30, bg_color="red")
    b.box(width=20, height=30, bg_color="green")
    b.box(width=20, height=30, bg_color="blue")

    slide = deck.new_slide(width=150, height=50)
    b = slide.box(
        row=True,
        height="100%",
        width="100%",
        bg_color="gray",
        justify_content="start",
        gap_x=30,
        gap_y=20,
    )
    b.box(width=20, height=30, bg_color="red")
    b.box(width=20, height=30, bg_color="green")
    b.box(width=20, height=30, bg_color="blue")


@check()
def test_expr_x_y_weight_height(deck):
    slide = deck.new_slide(width=200, height=80)
    box = slide.box(width=180, height=40, bg_color="green")
    slide.box(
        x=box.x(0.33),
        y=box.y(),
        width=box.width(0.33),
        height=box.height(0.5),
        bg_color="blue",
    )


@check(n_slides=2)
def test_m_xy_p_xy(deck):
    slide = deck.new_slide(width=100, height=100)
    box = slide.box(bg_color="green")
    box.box(width=30, height=30, bg_color="red").margin(x=10, y=20)

    slide = deck.new_slide(width=100, height=100)
    box = slide.box(bg_color="green").padding(x=20, y=10)
    box.box(width=30, height=30, bg_color="red")


@check()
def test_layout_grid_in_pixels(deck):
    slide = deck.new_slide(width=300, height=100)
    box = slide.box(
        grid=G(template_rows=[20, 35, 20], template_columns=[40, 100, 100]),
        bg_color="gray",
    )

    box.box(grid=G(column=1, row=2), bg_color="green")
    box.box(grid=G(column=2, row=1), bg_color="blue")
    box.box(grid=G(column=-2, row=(1, "span 3")), bg_color="orange")


@check()
def test_layout_simple_grid_fractions(deck):
    slide = deck.new_slide(width=300, height=100)
    box = slide.box(
        grid=G(template_rows=["1 fr", "1fr"], template_columns=["1 fr", "1 fr"]),
        bg_color="gray",
        width="250",
        height="90",
    )
    box.box(grid=G(column=1, row=1), bg_color="red")


@check()
def test_layout_grid_fractions(deck):
    slide = deck.new_slide(width=300, height=100)
    box = slide.box(
        width="90%",
        height="90%",
        grid=G(template_rows=["1fr", "2 fr", "1 fr"], template_columns=["50%", "1fr", "1fr"]),
        bg_color="gray",
    )

    box.box(grid=G(column=2, row=1), bg_color="green")
    box.box(grid=G(column=3, row=(1, 3)), bg_color="blue")

    box.box(grid=G(column=1, row=2), bg_color="orange")
    box.box(grid=G(column=2, row=2), bg_color="black")

    box.box(grid=G(column=(1, 4), row=3), bg_color="red")


@check()
def test_layout_grid_table(deck):
    data = [
        ("Name", "Time", "Type"),
        ("Jane", 3.5, "A1"),
        ("John", 4.1, "B7"),
        ("Johanna", 12.0, "C1"),
        ("Elise", 12.5, "D4"),
        ("Max", 320.2, "E1"),
    ]

    slide = deck.new_slide(width=1000, height=300)
    table = slide.box(
        width="70%",
        grid=G(template_columns=["2fr", "1fr", 130], template_rows=[50] + [40] * (len(data) - 1)),
        bg_color="#ddd",
    )
    header_style = TextStyle(weight=800)
    table.box(grid=G(column=(1, 4), row=1), bg_color="#fbc")
    for i in range(2, len(data) + 1, 2):
        table.box(grid=G(column=(1, 4), row=i), bg_color="#eee")
    column1 = table.box(grid=G(column=2, row=(1, len(data) + 1)))
    stroke = Stroke(color="#888", width=2)
    column1.add(Path(stroke=stroke).move_to(Point(0, 0)).line_to(Point(0, "100%")))
    column1.add(Path(stroke=stroke).move_to(Point("100%", 0)).line_to(Point("100%", "100%")))

    # Fill the table with data
    for i, row in enumerate(data, 1):
        s = header_style if i == 1 else None
        table.box(grid=G(column=1, row=i)).text(row[0], s)
        table.box(grid=G(column=2, row=i), row=True, justify_content="end", m_right=30).text(str(row[1]), s)
        table.box(grid=G(column=3, row=i), row=True, justify_content="start", m_left=30).text(row[2], s)
