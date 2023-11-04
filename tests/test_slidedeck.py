import os


def test_simple_box(tmp_path, deck):
    out_svg = tmp_path / "svgs"
    out_png = tmp_path / "pngs"

    slide = deck.new_slide()
    slide.box(width=400, height=200, bg_color="blue")
    slide.box(width=200, height=400, bg_color="red")
    deck.new_slide()
    deck.render(
        output_pdf="out.pdf", output_svg=out_svg, output_png=out_png, debug=True
    )

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
