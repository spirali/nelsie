import os
import sys

import pytest

PYTEST_DIR = os.path.dirname(os.path.abspath(__file__))
ROOT_DIR = os.path.dirname(PYTEST_DIR)
NELSIE_BIN = os.path.join(ROOT_DIR, "nelsie-builder", "target", "debug", "nelsie-builder")
CHECKS_DIR = os.path.join(PYTEST_DIR, "checks")
ASSETS_DIR = os.path.join(PYTEST_DIR, "assets")

sys.path.insert(0, os.path.join(ROOT_DIR, "nelsie-api"))


from nelsie import Resources, SlideDeck  # noqa

# To speedup tests, load resources just once
resources = Resources()


@pytest.fixture()
def deck():
    return SlideDeck(image_directory=ASSETS_DIR, resources=resources)
