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
