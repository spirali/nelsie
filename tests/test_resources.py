def test_resources_syntaxes(resources):
    syntaxes = resources.syntaxes()
    s = dict(syntaxes)
    assert "Python" in s
    assert "Rust" in s
    assert "C++" in s


def test_resources_themes(resources):
    themes = resources.themes()
    assert "InspiredGitHub" in themes
