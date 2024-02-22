from testutils import check

from nelsie import InSteps, TextStyle


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


@check(n_slides=3)
def test_active_steps(deck):
    slide = deck.new_slide(width=200, height=200)
    slide.box(width=30, height=30, bg_color="red")
    slide.box(width=30, height=30, bg_color="green", active="2")
    slide.box(width=30, height=30, bg_color="orange", active=3)
    slide.box(width=30, height=30, bg_color="blue")


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


@check(n_slides=4)
def test_step_global_counter(deck):
    deck.set_style("default", TextStyle(size=12))
    deck.set_style("g", TextStyle(color="green"))
    deck.set_style("r", TextStyle(color="red"))
    slide = deck.new_slide(width=100, height=40)
    slide.text("$(global_slide)/$(global_slides)  $(global_page)/$(global_pages)", parse_counters=True, bg_color="gray")
    slide.set_n_steps(2)
    slide = deck.new_slide(width=400, height=100)
    slide.text("$(global_slide)/$(global_slides)  $(global_page)/$(global_pages)", bg_color="gray")
    slide = deck.new_slide(width=100, height=40)
    slide.text(
        "$(global_slide)/$(global_slides) ~g{!!!} ~r{$(global_page)}/$(global_pages) ~g{!!!}",
        parse_counters=True,
        bg_color="gray",
    )


@check(n_slides=6)
def test_step_other_counter(deck):
    def create_slide(counters=None):
        slide = deck.new_slide(width=100, height=40, counters=counters)
        slide.text("$(my_slide)/$(my_slides)  $(my_page)/$(my_pages)", parse_counters=True, bg_color="gray")
        return slide

    deck.set_style("default", TextStyle(size=12))
    create_slide()
    create_slide(counters=["my"])
    create_slide(counters=["other"])
    slide = create_slide(counters=["my"])
    slide.set_n_steps(3)
    create_slide()
    slide.set_n_steps(2)


@check()
def test_step_invalid_counter(deck):
    deck.set_style("default", TextStyle(size=12))
    deck.set_style("g", TextStyle(color="green"))
    slide = deck.new_slide(width=100, height=40)
    slide.text("$(global_~g{page})", bg_color="gray", parse_counters=True)


@check(n_slides=4)
def test_show_next_last_keywords(deck):
    deck.set_style("default", TextStyle(size=12))
    slide = deck.new_slide(width=100, height=100)
    slide.box(show="last").text("Last")
    slide.box(show="next").text("Next")
    slide.box(show="last+").text("Last+")
    slide.box(show="next+").text("Next+")
    slide.box(show=3).text("Jump")
    slide.box(show="last").text("Last2")
    slide.box(show="next").text("Next2")


@check(n_slides=4)
def test_active_next_last_keywords(deck):
    deck.set_style("default", TextStyle(size=12))
    slide = deck.new_slide(width=100, height=100)
    slide.box(active="last").text("Last")
    slide.box(active="next").text("Next")
    slide.box(active="last+").text("Last+")
    slide.box(active="next+").text("Next+")
    slide.box(active=3).text("Jump")
    slide.box(active="last").text("Last2")
    slide.box(active="next").text("Next2")
