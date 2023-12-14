from testutils import check

from nelsie.layoutexpr import ConstExpr, SumExpr


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

    make_slide(
        deck.new_slide().box(width="100%", height="100%", row=False, reverse=False)
    )
    make_slide(
        deck.new_slide().box(width="100%", height="100%", row=False, reverse=True)
    )
    make_slide(
        deck.new_slide().box(width="100%", height="100%", row=True, reverse=False)
    )
    make_slide(
        deck.new_slide().box(width="100%", height="100%", row=True, reverse=True)
    )


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


def test_layout_expr():
    a = ConstExpr(10.0)
    b = ConstExpr(20.0)
    c = ConstExpr(35.0)
    assert a + b == SumExpr([a, b])
    assert a + b + c == SumExpr([a, b, c])


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
