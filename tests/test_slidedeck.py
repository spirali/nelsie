import os.path

from nelsie import SlideDeck
from conftest import ROOT_DIR
from testutils import check


def test_render_outputs(tmp_path, deck):
    out_svg = tmp_path / "svgs"
    out_png = tmp_path / "pngs"
    out_pdf = tmp_path / "out.pdf"

    slide = deck.new_slide()
    slide.box(width=400, height=200, bg_color="blue")
    slide.box(width=200, height=400, bg_color="red")

    slide = deck.new_slide(bg_color="green")
    slide.box(width=None, height=None, bg_color="black")
    deck.render(output_pdf=out_pdf, output_svg=out_svg, output_png=out_png, debug=True)

    with open(out_svg / "0-1.svg") as f:
        data = f.read()
        assert data.startswith(
            """<svg width="1024" height="768" viewBox="0 0 1024 768" xmlns="http://www.w3.org/2000/svg">"""
        )

    with open(out_svg / "1-1.svg") as f:
        data = f.read()
        assert data.startswith(
            """<svg width="1024" height="768" viewBox="0 0 1024 768" xmlns="http://www.w3.org/2000/svg">"""
        )

    with open(out_png / "0-1.png", "rb") as f:
        data = f.read(4)
        assert data == b"\x89PNG"

    with open(out_png / "1-1.png", "rb") as f:
        data = f.read(4)
        assert data == b"\x89PNG"

    with open(out_pdf, "rb") as f:
        data = f.read(4)
        assert data == b"%PDF"


@check(n_slides=2)
def test_slide_decorator(deck):
    @deck.slide()
    def my_slide(slide):
        slide.box(width=400, height=200, bg_color="blue")
        slide.box(width=200, height=400, bg_color="red")

    @deck.slide(width=300, height=120, debug_layout="green")
    def my_slide2(slide):
        slide.text("Hello world!")
