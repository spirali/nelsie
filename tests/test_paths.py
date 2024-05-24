from testutils import check

from nelsie import Arrow, InSteps, Path, Stroke, TextStyle


@check()
def test_render_paths(deck):
    slide = deck.new_slide(width=200, height=200)

    path = (
        Path(stroke=Stroke(color="orange", width=10.0))
        .move_to(0, 0)
        .line_to(50, 50)
        .line_to(50, 100)
        .quad_to(150, 100, 150, 150)
        .cubic_to(180, 190, 150, 190, 50, 150)
    )
    slide.box(width="90%", height="90%").draw(path)


@check()
def test_render_cubic(deck):
    slide = deck.new_slide(width=200, height=200)

    path = Path(stroke=Stroke(color="red", width=5.0)).move_to(10, 10).cubic_to(10, 190, 190, 190, 190, 10)
    slide.box(width="100%", height="100%").draw(path)


@check(n_slides=3)
def test_render_path_steps(deck):
    slide = deck.new_slide(width=200, height=200)

    path1 = Path(stroke=Stroke(color="orange", width=10.0)).move_to(0, 0).line_to(50, 50).line_to(50, 100)
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
        Path(stroke=Stroke(color="blue", width=5.0, dash_array=[20, 5], dash_offset=10)).move_to(0, 0).line_to(75, 150)
    )
    slide.box(width="90%", height="90%").draw([path1, path2])


@check()
def test_bg_color_opacity(deck):
    slide = deck.new_slide(width=100, height=100)
    slide.box(x=0, y=0, width=75, height=75, z_level=1, bg_color="#ff000060")
    slide.box(x=25, y=25, width=75, height=75, bg_color="#00ff0060")


@check()
def test_path_opacity(deck):
    slide = deck.new_slide(width=100, height=100)
    slide.draw(
        [
            Path(stroke=Stroke(color="#ff000060", width=30)).move_to(50, 0).line_to(50, 100),
            Path(stroke=Stroke(color="#0000ff60", width=20)).move_to(0, 50).line_to(100, 50),
        ]
    )


@check()
def test_path_box_positions(deck):
    slide = deck.new_slide(width=150, height=150)
    b1 = slide.box(width=20, height=30, bg_color="green")
    slide.box(width=30, height=40)
    b2 = slide.box(width=30, height=30, bg_color="green")

    slide.draw(
        [
            Path(stroke=Stroke(color="black", width=3))
            .move_to(b2.x(), b2.y() + 10)
            .line_to(b2.x() - 30, b2.y() + 10)
            .line_to(b2.x() - 30, b1.y(0.0))
            .line_to(b1.x(1.0), b1.y(0.5))
        ]
    )


@check()
def test_path_text_line_positions(deck):
    slide = deck.new_slide(width=160, height=200)

    text = slide.text("Hello world!\nNew line", TextStyle(size=10))

    slide.draw(
        [
            Path(stroke=Stroke(color="red", width=1)).move_to(20, 190).line_to(text.line_x(0), text.line_y(0)),
            Path(stroke=Stroke(color="green", width=1)).move_to(20, 190).line_to(text.line_x(1), text.line_y(1)),
            Path(stroke=Stroke(color="orange", width=1))
            .move_to(20, 190)
            .line_to(text.line_x(0, 1.0), text.line_y(0, 1.0)),
            Path(stroke=Stroke(color="teal", width=1))
            .move_to(20, 190)
            .line_to(text.line_x(1, 1.0), text.line_y(1, 1.0)),
        ]
    )


@check(n_slides=2)
def test_path_arrows(deck):
    arrow1 = Arrow(size=30, angle=40)
    arrow2 = Arrow(size=20, angle=10, color="red")
    arrow3 = Arrow(size=30, angle=20, stroke_width=2)
    arrow4 = Arrow(size=30, angle=45, stroke_width=2, color="blue")
    arrow5 = Arrow(size=24, angle=45, inner_point=2)
    arrow6 = Arrow(size=24, angle=45, inner_point=0.5)

    slide = deck.new_slide(width=150, height=280)
    slide.draw(
        [
            Path(stroke=Stroke(color="green", width=3), arrow_end=arrow1).move_to(20, 20).line_to(100, 50),
            Path(stroke=Stroke(color="black", width=3), arrow_end=arrow2).move_to(20, 60).line_to(100, 90),
            Path(stroke=Stroke(color="black", width=3), arrow_end=arrow3).move_to(20, 100).line_to(100, 130),
            Path(stroke=Stroke(color="black", width=3), arrow_end=arrow4).move_to(20, 140).line_to(100, 170),
            Path(stroke=Stroke(color="black", width=3), arrow_end=arrow5).move_to(20, 170).line_to(100, 210),
            Path(stroke=Stroke(color="black", width=3), arrow_end=arrow6).move_to(20, 210).line_to(100, 250),
        ]
    )
    slide = deck.new_slide(width=150, height=280)
    slide.draw(
        [
            Path(stroke=Stroke(color="green", width=3), arrow_start=arrow1).move_to(20, 20).line_to(100, 50),
            Path(stroke=Stroke(color="black", width=3), arrow_start=arrow2).move_to(20, 60).line_to(100, 90),
            Path(stroke=Stroke(color="black", width=3), arrow_start=arrow3).move_to(20, 100).line_to(100, 130),
            Path(stroke=Stroke(color="black", width=3), arrow_start=arrow4).move_to(20, 140).line_to(100, 170),
            Path(stroke=Stroke(color="black", width=3), arrow_start=arrow5).move_to(20, 170).line_to(100, 210),
            Path(stroke=Stroke(color="black", width=3), arrow_start=arrow6).move_to(20, 210).line_to(100, 250),
        ]
    )


@check()
def test_path_close(deck):
    slide = deck.new_slide(width=150, height=170)
    slide.draw(
        [
            Path(stroke=Stroke(color="green", width=3), fill_color="gray")
            .move_to(20, 20)
            .line_to(120, 50)
            .line_to(70, 150)
            .close()
        ]
    )


@check()
def test_path_oval(deck):
    slide = deck.new_slide(width=150, height=150)
    slide.draw(
        [
            Path.oval(
                25,
                25,
                125,
                125,
                stroke=Stroke(color="green", width=3),
                fill_color="gray",
            ),
            Path.oval(10, 70, 140, 90, fill_color="red"),
        ]
    )
