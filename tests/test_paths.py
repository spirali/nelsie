from nelsie import Path, Stroke
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