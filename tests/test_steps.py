from nelsie.steps import parse_bool_steps

from nelsie.steps_extract import extract_steps
from testutils import check

from nelsie import TextStyle, StepVal, Box, SlideCounter


def test_step_value():
    a = StepVal().at(2, "A").at(4, "B")
    assert a.get_step(1) is None
    assert a.get_step(2) == "A"
    assert a.get_step(3) == "A"
    assert a.get_step(4) == "B"
    assert a.get_step(5) == "B"

    assert a.get_step((1, 9)) is None
    assert a.get_step((2, 1)) == "A"
    assert a.get_step((3, 9, 9, 9, 9)) == "A"
    assert a.get_step((4, 0, 0)) == "B"


def test_parse_bool_steps():
    s = parse_bool_steps("1")
    assert s.values == {1: True, 2: False}
    assert s.named_steps == [1]
    s = parse_bool_steps("1, 2, 3")
    assert s.values == {1: True, 4: False}
    assert set(s.named_steps) == {1, 2, 3}

    s = parse_bool_steps("2+")
    assert s.values == {2: True}
    assert set(s.named_steps) == {2}

    s = parse_bool_steps("2, 4+")
    assert s.values == {2: True, 3: False, 4: True}
    assert set(s.named_steps) == {2, 4}

    s = parse_bool_steps("4, 10-20, 30-40, 50+")
    assert s.values == {4: True, 5: False, 10: True, 21: False, 30: True, 41: False, 50: True}
    assert set(s.named_steps) == {4, 10, 20, 30, 40, 50}

    s = parse_bool_steps("2.5.1+")
    assert s.values == {(2, 5, 1): True}
    assert set(s.named_steps) == {(2, 5, 1)}

    s = parse_bool_steps("2.5.1")
    assert s.values == {(2, 5, 1): True, (2, 5, 2): False}
    assert set(s.named_steps) == {(2, 5, 1)}

    s = parse_bool_steps("!20.50.100")
    assert s.values == {(20, 50, 100): True, (20, 50, 100, 0): False}
    assert set(s.named_steps) == {(20, 50, 100)}

    s = parse_bool_steps("3-!6")
    assert s.values == {3: True, (6, 0): False}
    assert set(s.named_steps) == {3, 6}

    s = parse_bool_steps("1, 2?, 3, 10-20?")
    assert s.values == {1: True, 4: False, 10: True, 21: False}
    assert set(s.named_steps) == {1, 3, 10}


def test_extract_steps():
    def check(obj, target):
        out = set()
        extract_steps(obj, out)
        assert out == target

    check(None, set())
    a = StepVal().at(2, "A").at(4, "B")
    b = StepVal().at(4, "A").at(5, "B")
    check(a, {2, 4})
    check([4, 5, 6, a, b], {2, 4, 5})
    check(a, {2, 4})
    check({1: a, 2: b}, {2, 4, 5})
    check(Box().text(a), {2, 4})


@check(n_slides=3)
def test_render_steps(deck):
    slide = deck.new_slide()
    slide.box(
        width=100,
        height=StepVal("75%").at(2, "25%"),
        bg_color=StepVal("red").at(2, "green").at(3, "blue"),
    )


@check(n_slides=3)
def test_render_substeps(deck):
    slide = deck.new_slide(init_steps=())
    slide.box(
        width=100,
        height=StepVal().at((2, 3, 1), "75%").at((2, 3, 2), "25%"),
        bg_color=StepVal().at((2, 3, 1), "red").at((2, 3, 2), "green").at(4, "blue"),
    )


@check(n_slides=4)
def test_show_steps(deck):
    slide = deck.new_slide(width=200, height=200)
    b = slide.box(show="2+", width=100, height=100, bg_color="red")
    b.box(show="1, 3", width=40, height=40, bg_color="blue")
    b.box(show="4", width=40, height=40, bg_color="green")


@check(n_slides=3)
def test_active_steps(deck):
    slide = deck.new_slide(width=200, height=200)
    slide.box(width=30, height=30, bg_color="red")
    slide.box(width=30, height=30, bg_color="green", active="2")
    slide.box(width=30, height=30, bg_color="orange", active=3)
    slide.box(width=30, height=30, bg_color="blue")


# @check(n_slides=4)
# def test_replace_steps(deck):
#     slide = deck.new_slide(width=200, height=200)
#     slide.box(
#         replace_steps={1: 2, 3: 1},
#         width=StepVal("100").at(2, "50").at(3, "20"),
#         height=20,
#         bg_color="green",
#     )
#     slide.image("test.svg", width="50%", replace_steps={1: 3, 2: 1})


# def test_set_get_steps(deck):
#     slide = deck.new_slide()
#     assert slide.get_steps() == [(1,)]
#     slide.box(width=InSteps({1: 100, 3: 200, 5: 600}))
#     slide.box(width=InSteps({5: 600, (5, 3): 300}))
#     assert slide.get_steps() == [(1,), (3,), (5,), (5, 3)]
#     slide.insert_step(3)
#     slide.insert_step((6, 2))
#     assert slide.get_steps() == [(1,), (3,), (5,), (5, 3), (6, 2)]
#
#     slide.remove_step(5)
#     assert slide.get_steps() == [(1,), (3,), (5, 3), (6, 2)]
#
#     slide.remove_steps_above((5, 3))
#     assert slide.get_steps() == [(1,), (3,), (5, 3)]
#
#     slide.remove_steps_below((5, 3))
#     assert slide.get_steps() == [(5, 3)]
#
#     slide.set_steps({1, (3, 2), 2})
#     assert slide.get_steps() == [(1,), (2,), (3, 2)]


def add_global_counter(slide, current, total):
    slide = slide.copy()

    n_slide = current["global"].slide
    n_slide_total = total["global"].slide

    n_page = current["global"].page
    n_page_total = total["global"].page

    slide.text(
        f"{n_slide}/{n_slide_total}  {n_page}/{n_page_total}",
        bg_color="gray",
    )
    return slide


@check(n_slides=4)
def test_step_global_counter(deck):
    deck.set_style("default", TextStyle(size=12))
    deck.set_style("g", TextStyle(color="green"))
    deck.set_style("r", TextStyle(color="red"))

    slide = deck.new_slide(width=100, height=40, postprocess_fn=add_global_counter)
    slide.insert_step(2)
    deck.new_slide(width=100, height=40)
    deck.new_slide(width=100, height=40, postprocess_fn=add_global_counter)


@check(n_slides=8)
def test_step_other_counter(deck):
    def add_counter(slide, current, total):
        slide = slide.copy()

        n_slide = current["my"].slide
        n_slide_total = total["my"].slide

        n_page = current["my"].page
        n_page_total = total["my"].page

        slide.text(
            f"{n_slide}/{n_slide_total}  {n_page}/{n_page_total}",
            bg_color="gray",
        )
        return slide

    def create_slide(counters=None):
        return deck.new_slide(width=100, height=40, counters=counters, postprocess_fn=add_counter)
        # slide.text(
        #     "$(my_slide)/$(my_slides)  $(my_page)/$(my_pages)",
        #     style=TextStyle(color=color),
        #     parse_counters=True,
        #     bg_color="gray",
        # )

    deck.set_style("default", TextStyle(size=12))
    create_slide()
    create_slide(counters=["my"])
    create_slide(counters=["other"])
    slide = create_slide(counters=["my"])
    slide.insert_step(2)
    slide.insert_step(3)
    slide = create_slide()
    slide.insert_step(2)


def test_slide_counter(deck):
    c = StepCounter()
    assert c.last() == 1
    assert c.last() == 1
    assert c.last_p() == "1+"
    assert c.next_p() == "2+"
    assert c.next() == 3
    assert c.last() == 3
    c.set(10)
    assert c.last() == 10
    assert c.next_p() == "11+"


@check(n_slides=9)
def test_subslides(deck):
    deck.set_style("default", TextStyle(size=8))

    slide = deck.new_slide(width=40, height=70, postprocess_fn=add_global_counter, name="main")
    slide.box(width=20, height=20, bg_color="red")
    slide.box(width=20, height=20, bg_color="orange", show="2+")
    slide.box(width=20, height=20, bg_color="green", show="3+")

    slide2 = slide.new_slide_at(
        3, width=40, height=40, postprocess_fn=add_global_counter, name="blue", debug_steps=True
    )
    slide2.box(width=10, height=10, bg_color="blue")
    slide2.box(width=10, height=10, bg_color="blue", show="2+")

    slide3 = slide.new_slide_at(3, width=40, height=40, postprocess_fn=add_global_counter)
    slide3.box(width=10, height=10, bg_color="purple")

    slide4 = slide2.new_slide_at(2, width=40, height=40, postprocess_fn=add_global_counter)
    slide4.box(width=5, height=5, bg_color="gray")


@check(n_slides=5)
def test_subslides_decorator(deck):
    deck.set_style("default", TextStyle(size=12))

    @deck.slide(width=100, height=50)
    def my_slide(slide):
        slide.box().text("One")
        slide.box(show="2+").text("Two")
        slide.box(show="3+").text("Three")

    @my_slide.slide_at(3, width=100, height=50)
    def inserted(slide):
        slide.text("Inserted")


@check(n_slides=5)
def test_debug_steps(deck):
    slide = deck.new_slide(debug_steps=True, width=300, height=300)
    slide.text("Hello", show="2+")
    slide.text("World", show="22, 3, 111")


@check(n_slides=2)
def test_ignore_steps(deck):
    slide = deck.new_slide(width=20, height=20)
    slide.box(
        width=20,
        height=20,
        bg_color=StepVal("red").at(2, "green").at(3, "blue").at(5, "magenta"),
    )
    slide.ignore_steps("2-4")
