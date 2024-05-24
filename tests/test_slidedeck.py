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

    svg_data = deck.render(None, "svg")
    assert len(svg_data) == 2

    assert deck.render(out_svg, "svg") is None

    with open(out_svg / "0.svg", "rb") as f:
        data = f.read()
        assert data.startswith(b"<svg ")
        assert svg_data[0] == (0, (1,), data)

    with open(out_svg / "1.svg", "rb") as f:
        data = f.read()
        assert data.startswith(b"<svg ")
        assert svg_data[1] == (1, (1,), data)

    png_data = deck.render(None, "png")
    assert len(png_data) == 2

    deck.render(out_png, "png")

    with open(out_png / "0.png", "rb") as f:
        data = f.read()
        assert data[:4] == b"\x89PNG"
        assert png_data[0] == (0, (1,), data)

    with open(out_png / "1.png", "rb") as f:
        data = f.read()
        assert data[:4] == b"\x89PNG"
        assert png_data[1] == (1, (1,), data)

    deck.render(out_pdf, "pdf")

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
