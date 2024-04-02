from conftest import new_resources, ASSETS_DIR

from nelsie import SlideDeck


def test_resources_syntaxes(resources):
    syntaxes = resources.syntaxes()
    s = dict(syntaxes)
    assert "Python" in s
    assert "Rust" in s
    assert "C++" in s

    r = new_resources(default_code_syntaxes=False)
    syntaxes = r.syntaxes()
    assert not syntaxes


def test_resources_add_syntax():
    r = new_resources(default_code_syntaxes=False)
    r.load_code_syntax_dir(ASSETS_DIR)
    syntaxes = dict(r.syntaxes())
    assert "testC" in syntaxes
    deck = SlideDeck(resources=r)
    slide = deck.new_slide()
    slide.code("if (x > 0) { return 1 }", "testC")


def test_resources_themes(resources):
    themes = resources.themes()
    assert "InspiredGitHub" in themes


def test_resources_add_theme():
    r = new_resources()
    r.load_code_theme_dir(ASSETS_DIR)
    themes = r.themes()
    assert "test" in themes
    deck = SlideDeck(resources=r)
    slide = deck.new_slide()
    slide.code("if (x > 0) { return 1 }", "C", theme="test")
