from testutils import check

from nelsie import InSteps


def test_step_values():
    def check(in_steps, values, n_steps):
        assert in_steps.in_step_values == values
        assert in_steps.n_steps == n_steps

    check(InSteps(["red", "red", "blue"]), {1: "red", 3: "blue"}, 3)

    check(
        InSteps({2: "black", 1: "orange", 4: "green"}),
        {2: "black", 1: "orange", 4: "green"},
        4,
    )


@check(n_slides=3)
def test_render_steps(deck):
    slide = deck.new_slide()
    slide.box(
        width=100,
        height=InSteps({1: "75%", 2: "25%"}),
        bg_color=InSteps(["red", "green", "blue"]),
    )


@check(n_slides=4)
def test_show_steps(deck):
    slide = deck.new_slide(width=200, height=200)
    b = slide.box(show="2+", width=100, height=100, bg_color="red")
    b.box(show="1, 3", width=40, height=40, bg_color="blue")
    b.box(show="4", width=40, height=40, bg_color="green")


@check(n_slides=4)
def test_replace_steps(deck):
    slide = deck.new_slide(width=200, height=200)
    slide.box(
        replace_steps={1: 2, 3: 1},
        width=InSteps({1: "100", 2: "50", 3: "20"}),
        height=20,
        bg_color="green",
    )
    slide.image("test.svg", width="50%", replace_steps={1: 3, 2: 1})


def test_set_get_n_steps(deck):
    slide = deck.new_slide()
    assert slide.get_n_steps() == 1
    slide.box(width=InSteps({1: 100, 3: 200, 5: 600}))
    assert slide.get_n_steps() == 5
    slide.set_n_steps(3)
    assert slide.get_n_steps() == 3
