from testutils import check

@check()
def test_render_raster_image_native_size(deck):
    slide = deck.new_slide(width=400, height=400)
    slide.image("testimg.png")
    slide.image("testimg.jpeg")

    deck.render(output_pdf="/tmp/out.pdf")
