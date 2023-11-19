from testutils import check


@check()
def test_render_raster_image_native_size(deck):
    slide = deck.new_slide(width=400, height=400)
    slide.image("testimg.png")
    slide.image("testimg.jpeg")


@check(n_slides=5)
def test_render_raster_image_forced_size(deck):
    slide = deck.new_slide(width=420, height=400)
    slide.image("testimg.png", width="100%", height="100%")

    slide = deck.new_slide(width=400, height=400)
    slide.image("testimg.png", width=80, bg_color="gray")
    slide.image("testimg.jpeg", width=80, bg_color="blue")

    slide = deck.new_slide(width=400, height=400)
    slide.image("testimg.png", height=80, bg_color="gray")
    slide.image("testimg.jpeg", height=80, bg_color="blue")

    slide = deck.new_slide(width=400, height=400)
    slide.image("testimg.png", width="80%", bg_color="gray")

    slide = deck.new_slide(width=400, height=400)
    slide.image("testimg.png", height="80%", bg_color="gray")


@check(n_slides=4)
def test_render_svg_image_steps(deck):
    slide = deck.new_slide(width=420, height=400)
    slide.image("test.svg", width="90%")


@check(n_slides=1)
def test_render_svg_image_no_steps(deck):
    slide = deck.new_slide(width=420, height=400)
    slide.image("test.svg", width="90%", enable_steps=False)


@check(n_slides=6)
def test_render_svg_image_shift(deck):
    slide = deck.new_slide(width=100, height=100)
    box = slide.box(width="90%", height="80%", bg_color="gray")
    box.image("test.svg", width="80%", shift_steps=2)
