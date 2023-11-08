from testutils import check
from nelsie import TextStyle


def test_text_styles():
    import dataclasses

    s = TextStyle(color="red")

    print(dataclasses.astuple(s))


@check()
def test_render_text(deck):
    slide = deck.new_slide()
    slide.box(bg_color="red")
    deck.render(output_pdf="/tmp/out.pdf")
