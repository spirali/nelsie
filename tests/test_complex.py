from conftest import ASSETS_DIR
from nelsie import Stroke, Path, Arrow
from testutils import check
import os


@check(n_slides=4, pdf_threshold=205.0)  # On CI there is some problem with pdf rasterization of this example
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
    slide.overlay(show="1-3", z_level=1).add(
        Path(stroke=Stroke(color="black", width=15), arrow_end=Arrow(30))
        .move_to(tiles[(3, 4)].p(0.5, 0.5))
        .line_to(tiles[(3, 2)].p(0.5, 0.5))
        .line_to(tiles[(4, 2)].p(0.5, 0.5))
    )

    knight = os.path.join(ASSETS_DIR, "knight.svg")

    # Draw knight
    tiles[(3, 4)].box(show="1", z_level=2).image(knight)
    tiles[(3, 3)].box(show="2", z_level=2).image(knight)
    tiles[(3, 2)].box(show="3", z_level=2).image(knight)
    tiles[(4, 2)].box(show="4", z_level=2).image(knight)
