from nelsie import InSteps
from nelsie.steps import parse_steps

import pytest

from testutils import check


def test_parse_steps():
    def p(obj):
        in_steps = parse_steps(obj)
        return in_steps.values, in_steps.n_steps

    assert p(3) == ([False, False, True, False], 3)
    assert p([1, 3, 5]) == ([True, False, True, False, True, False], 5)
    assert p([1, 2]) == ([True, True, False], 2)

    with pytest.raises(ValueError, match="Step cannot be a zero or negative integer"):
        p(0)
    with pytest.raises(ValueError, match="Step cannot be a zero or negative integer"):
        p([0])

    assert p("10") == (9 * [False] + [True, False], 10)
    assert p("1,2,4") == ([True, True, False, True, False], 4)
    assert p("2-4,7") == (
        [
            False,
            True,
            True,
            True,
            False,
            False,
            True,
            False,
        ],
        7,
    )

    assert p("2-4,7+") == ([False, True, True, True, False, False, True], 7)
    assert p("3+") == ([False, False, True], 3)
    assert p("3,2 , 1") == ([True, True, True, False], 3)


def test_step_values():
    def check(in_steps, values, n_steps):
        assert in_steps.values == values
        assert in_steps.n_steps == n_steps

    check(InSteps(["red", "red", "blue"]), ["red", "red", "blue"], 3)

    with pytest.raises(ValueError, match="cannot be an empty list"):
        InSteps([])

    check(
        InSteps({"2": "black", (1, 3): "orange", "4+": "green"}),
        ["orange", "black", "orange", "green"],
        4,
    )

    check(
        InSteps({"2": "black", "1,3,4+": "green"}),
        ["green", "black", "green", "green"],
        4,
    )

    with pytest.raises(ValueError, match="Multiple definitions assigned for step 4"):
        print(InSteps({"4": "black", "1,2,3+": "green"}).values)

    with pytest.raises(ValueError, match="Value not defined for step 2"):
        InSteps({"4+": "black", "1,3": "green"})

    with pytest.raises(
        ValueError,
        match="Multiple definitions assigned for step 3",
    ):
        InSteps({"2+": "black", "1,3+": "green"})


@check(n_slides=3)
def test_render_steps(deck):
    slide = deck.new_slide()
    slide.box(
        width=100,
        height=InSteps({1: "75%", "2+": "25%"}),
        bg_color=InSteps(["red", "green", "blue"]),
    )


@check(n_slides=4)
def test_show_steps(deck):
    slide = deck.new_slide(width=200, height=200)
    b = slide.box(show="2+", width=100, height=100, bg_color="red")
    b.box(show="1, 3", width=40, height=40, bg_color="blue")
    b.box(show="4", width=40, height=40, bg_color="green")
