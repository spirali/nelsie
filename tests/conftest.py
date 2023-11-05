import sys
import os
import pytest

PYTEST_DIR = os.path.dirname(os.path.abspath(__file__))
ROOT_DIR = os.path.dirname(PYTEST_DIR)
NELSIE_BIN = os.path.join(ROOT_DIR, "target", "debug", "nelsie")
CHECKS_DIR = os.path.join(PYTEST_DIR, "assets", "checks")

sys.path.insert(0, os.path.join(ROOT_DIR, "python"))

from nelsie import SlideDeck  # noqa


@pytest.fixture()
def deck():
    return SlideDeck(nelsie_bin=NELSIE_BIN)
