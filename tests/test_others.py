from testutils import check


@check()
def test_box_debug_frame(deck):
    slide = deck.new_slide(width=200, height=150)
    slide.box(width=180, height="30%", debug_layout=True, name="Box 123")
    slide.box(width=100, height="30%", debug_layout="orange")
    slide.box(width=100, height="30%")
    slide.box(debug_layout=True)

    # slide = deck.new_slide(width=200, height=150, debug_frames=True)
    # b = slide.box(width="80%", height="80%")
    # b = b.box(width="100%", height="100%")
    # b = b.box(width="99%", height="99%")
    # b.box(width="99%", height="99%")
    #
    # deck.render(output_pdf="/tmp/out.pdf")


@check()
def test_box_border_radius(deck):
    slide = deck.new_slide(width=150, height=150)
    slide.box(x=75, y=20, width=35, height=120, bg_color="red", border_radius=10)
    slide.box(x=10, y=50, width=100, height=10, bg_color="green", border_radius=5)


@check()
def test_pdf_url(deck):
    # Mostly just smoke test, as I do not know how to test annotations in resulting PDF
    slide = deck.new_slide(width=100, height=100)
    slide.box(
        x=65, y=10, width=32, height=20, bg_color="red", border_radius=10, url="https://github.com/spirali/nelsie"
    )
