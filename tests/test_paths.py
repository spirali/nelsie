from nelsie import Path, Stroke, InSteps
from testutils import check


@check()
def test_render_paths(deck):
    slide = deck.new_slide(width=200, height=200)

    path = (
        Path(stroke=Stroke(color="orange", width=10.0))
        .move_to(0, 0)
        .line_to(50, 50)
        .line_to(50, 100)
        .quad_to(150, 100, 150, 150)
        .cubic_to(180, 180, 190, 190, 50, 150)
    )
    slide.box(width="90%", height="90%").draw(path)


@check(n_slides=4)
def test_render_path_steps(deck):
    slide = deck.new_slide(width=200, height=200)

    path1 = (
        Path(stroke=Stroke(color="orange", width=10.0))
        .move_to(0, 0)
        .line_to(50, 50)
        .line_to(50, 100)
    )
    path2 = Path(stroke=Stroke(color="green", width=5.0)).move_to(0, 0).line_to(75, 150)
    slide.box(width="90%", height="90%").draw(InSteps({2: path1, 4: path2}))


@check(n_slides=1)
def test_render_path_dash(deck):
    slide = deck.new_slide(width=200, height=200)

    path1 = (
        Path(stroke=Stroke(color="green", width=10.0, dash_array=[10, 5, 5, 7]))
        .move_to(0, 0)
        .line_to(100, 50)
        .line_to(50, 100)
    )
    path2 = (
        Path(stroke=Stroke(color="blue", width=5.0, dash_array=[20, 5], dash_offset=10))
        .move_to(0, 0)
        .line_to(75, 150)
    )
    slide.box(width="90%", height="90%").draw([path1, path2])
    deck.render(output_pdf="/tmp/out.pdf")
