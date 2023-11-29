import pytest
from nelsie.export import FractionSize, PointsSize
from nelsie.parsers import parse_size


def test_parse_size():
    assert parse_size(2) == PointsSize(2)
    assert parse_size(123.25) == PointsSize(123.25)
    assert parse_size("0") == PointsSize(0)
    assert parse_size("123") == PointsSize(123)

    assert parse_size("0%") == FractionSize(0.0)
    assert parse_size("12%") == FractionSize(0.12)
    assert parse_size("12.234%") == FractionSize(0.12234)

    assert parse_size(None) is None

    with pytest.raises(ValueError, match="Invalid length definition"):
        parse_size("xxx")
    with pytest.raises(ValueError, match="Invalid length definition"):
        parse_size("12%%")
    with pytest.raises(ValueError, match="Invalid length definition"):
        parse_size([10])
