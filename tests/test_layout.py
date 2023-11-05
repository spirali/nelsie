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
