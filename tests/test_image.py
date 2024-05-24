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
    slide.insert_step(2)
    slide.insert_step(3)
    box.image("test.svg", width="80%", shift_steps=2)


@check(n_slides=3)
def test_render_ora_image_steps(deck):
    slide = deck.new_slide(width=250, height=250)
    box = slide.box(bg_color="gray")
    box.image("test.ora")


@check(n_slides=1)
def test_render_ora_image_no_steps(deck):
    slide = deck.new_slide(width=250, height=250)
    box = slide.box(bg_color="gray")
    box.image("test.ora", enable_steps=False)


@check(n_slides=1)
def test_render_ora_image_scale(deck):
    slide = deck.new_slide(width=400, height=400)
    slide.image("test.ora", width="80%", enable_steps=False, bg_color="gray")
    slide.image("test.ora", height="20%", enable_steps=False, bg_color="gray")


@check()
def test_svg_image_dtd(deck):
    slide = deck.new_slide(width=40, height=40)
    slide.image("knight_with_dtd.svg")
