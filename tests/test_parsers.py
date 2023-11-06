from nelsie.parsers import parse_size
import pytest


def test_parse_size():
    assert parse_size(2) == {"points": 2}
    assert parse_size(123.25) == {"points": 123.25}
    assert parse_size("0") == {"points": 0}
    assert parse_size("123") == {"points": 123.0}

    assert parse_size("0%") == {"percent": 0}
    assert parse_size("12%") == {"percent": 12.0}
    assert parse_size("12.234%") == {"percent": 12.234}

    assert parse_size("auto") == "auto"

    with pytest.raises(ValueError, match="Invalid size definition"):
        parse_size("xxx")
    with pytest.raises(ValueError, match="Invalid size definition"):
        parse_size("12%%")
    with pytest.raises(ValueError, match="Invalid size definition"):
        parse_size([10])
