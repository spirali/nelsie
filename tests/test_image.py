import os

from nelsie import StepVal
from testutils import check
from conftest import ASSETS_DIR

IMG_PNG = os.path.join(ASSETS_DIR, "testimg.png")
IMG_JPEG = os.path.join(ASSETS_DIR, "testimg.jpeg")
IMG_SVG = os.path.join(ASSETS_DIR, "test.svg")
IMG_SVG_WITH_DEFS = os.path.join(ASSETS_DIR, "with_defs.svg")
IMG_ORA = os.path.join(ASSETS_DIR, "test.ora")


@check()
def test_render_raster_image_native_size(deck):
    slide = deck.new_slide(width=400, height=400)
    slide.image(IMG_PNG)
    slide.image(IMG_JPEG)


@check(n_slides=6)
def test_render_raster_image_forced_size(deck):
    slide = deck.new_slide(width=420, height=400)
    slide.image(IMG_PNG, width="100%", height="100%")

    slide = deck.new_slide(width=400, height=400)
    slide.image(IMG_PNG, width=80, bg_color="gray")
    slide.image(IMG_JPEG, width=80, bg_color="blue")

    slide = deck.new_slide(width=400, height=400)
    slide.image(IMG_PNG, height=80, bg_color="gray")
    slide.image(IMG_JPEG, height=80, bg_color="blue")

    slide = deck.new_slide(width=400, height=400)
    slide.image(IMG_PNG, width="80%", bg_color="gray")

    slide = deck.new_slide(width=400, height=400)
    slide.image(IMG_PNG, height="80%", bg_color="gray")

    slide = deck.new_slide(width=420, height=400)
    box = slide.box(width="100%", height="40")
    box.image(IMG_SVG, width="100%", height="100%", enable_steps=False)


@check(n_slides=4)
def test_render_svg_image_steps(deck):
    slide = deck.new_slide(width=420, height=400)
    slide.image(IMG_SVG, width="90%")


@check(n_slides=1)
def test_render_svg_image_no_steps(deck):
    slide = deck.new_slide(width=420, height=400)
    slide.image(IMG_SVG, width="90%", enable_steps=False)


@check(n_slides=2)
def test_render_svg_with_defs(deck):
    slide = deck.new_slide(width=300, height=300)
    slide.image(IMG_SVG_WITH_DEFS, width="90%")


@check(n_slides=6)
def test_render_svg_image_shift(deck):
    slide = deck.new_slide(width=100, height=100)
    box = slide.box(width="90%", height="80%", bg_color="gray")
    slide.insert_step(2)
    slide.insert_step(3)
    box.image(IMG_SVG, width="80%", shift_steps=2)


@check(n_slides=3)
def test_render_ora_image_steps(deck):
    slide = deck.new_slide(width=250, height=250)
    box = slide.box(bg_color="gray")
    box.image(IMG_ORA)


@check(n_slides=1)
def test_render_ora_image_no_steps(deck):
    slide = deck.new_slide(width=250, height=250)
    box = slide.box(bg_color="gray")
    box.image(IMG_ORA, enable_steps=False)


@check(n_slides=1)
def test_render_ora_image_scale(deck):
    slide = deck.new_slide(width=400, height=400)
    slide.image(IMG_ORA, width="80%", enable_steps=False, bg_color="gray")
    slide.image(IMG_ORA, height="20%", enable_steps=False, bg_color="gray")


@check()
def test_svg_image_dtd(deck):
    slide = deck.new_slide(width=40, height=40)
    slide.image(os.path.join(ASSETS_DIR, "knight_with_dtd.svg"), width="40")
    print(deck.render(None, "layout"))


@check(n_slides=5)
def test_image_path_in_steps(deck):
    slide = deck.new_slide(width=150, height=150)
    slide.image(None)
    slide.image(
        StepVal(IMG_JPEG).at(2, IMG_PNG).at(4, None).at(5, IMG_JPEG),
        width=100,
        # height=100,
    )
    slide.insert_step(3)


@check()
def test_inline_image(deck):
    slide = deck.new_slide(width=200, height=200)
    with open(IMG_PNG, "rb") as f:
        png_data = f.read()
    with open(IMG_JPEG, "rb") as f:
        jpeg_data = f.read()
    with open(IMG_SVG, "r") as f:
        svg_data = f.read()
    slide.image((png_data, "png"), height="30%")
    slide.image((jpeg_data, "jpeg"), height="30%")
    slide.image((svg_data, "svg"), enable_steps=False, height="30%")
