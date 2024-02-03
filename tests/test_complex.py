from nelsie import Stroke, Path, Arrow
from testutils import check


@check(n_slides=4)
def test_chessboard(deck):
    slide = deck.new_slide(width=600, height=600)

    # Draw chessboard
    colors = ["orange", "#665566"]
    tiles = {}
    board = slide.box(width=500, height=500)
    for i in range(8):
        row = board.box(width="100%", flex_grow=1.0, row=True)
        for j in range(8):
            b = row.box(height="100%", flex_grow=1.0, bg_color=colors[(i + j) % 2])
            tiles[(j, i)] = b.overlay(z_level=0)

    # Draw arrow
    slide.overlay(show="1-3", z_level=1).draw(
        Path(stroke=Stroke(color="black", width=15), arrow_end=Arrow(30))
        .move_to(tiles[(3, 4)].x(0.5), tiles[(3, 4)].y(0.5))
        .line_to(tiles[(3, 2)].x(0.5), tiles[(3, 2)].y(0.5))
        .line_to(tiles[(4, 2)].x(0.5), tiles[(4, 2)].y(0.5))
    )

    # Draw knight
    tiles[(3, 4)].box(show="1", z_level=2).image("knight.svg")
    tiles[(3, 3)].box(show="2", z_level=2).image("knight.svg")
    tiles[(3, 2)].box(show="3", z_level=2).image("knight.svg")
    tiles[(4, 2)].box(show="4", z_level=2).image("knight.svg")
