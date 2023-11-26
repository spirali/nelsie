import pytest
from nelsie.steps.insteps import parse_steps
from testutils import check

from nelsie import InSteps


def test_parse_steps():
    def p(obj):
        in_steps = parse_steps(obj)
        return in_steps.values, in_steps.n_steps

    assert p(3) == ({1: False, 3: True, 4: False}, 3)
    assert p([1, 3, 5]) == (
        {1: True, 2: False, 3: True, 4: False, 5: True, 6: False},
        5,
    )
    assert p([1, 2]) == ({1: True, 3: False}, 2)

    with pytest.raises(ValueError, match="Step cannot be a zero or negative integer"):
        p(0)
    with pytest.raises(ValueError, match="Step cannot be a zero or negative integer"):
        p([0])

    assert p("10") == ({1: False, 10: True, 11: False}, 10)
    assert p("1,2,4") == ({1: True, 3: False, 4: True, 5: False}, 4)
    assert p("2-4,7") == (
        {
            1: False,
            2: True,
            5: False,
            7: True,
            8: False,
        },
        7,
    )

    assert p("2-4,7+") == ({1: False, 2: True, 5: False, 7: True}, 7)
    assert p("3+") == ({1: False, 3: True}, 3)
    assert p("3,2 , 1") == ({1: True, 4: False}, 3)


def test_step_values():
    def check(in_steps, values, n_steps):
        assert in_steps.values == values
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
