def test_render_paths(deck):
    slide = deck.new_slide(width=200, height=200)

    path = Path(...).move_to(0, 0).line_to()
    slide.draw(path)
    deck.render(output_pdf="/tmp/out.pdf")
