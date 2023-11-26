import os
import sys

import pytest

PYTEST_DIR = os.path.dirname(os.path.abspath(__file__))
ROOT_DIR = os.path.dirname(PYTEST_DIR)
NELSIE_BIN = os.path.join(ROOT_DIR, "nelsie-builder", "target", "debug", "nelsie")
CHECKS_DIR = os.path.join(PYTEST_DIR, "checks")
ASSETS_DIR = os.path.join(PYTEST_DIR, "assets")

sys.path.insert(0, os.path.join(ROOT_DIR, "nelsie-api"))


from nelsie import SlideDeck  # noqa


@pytest.fixture()
def deck():
    return SlideDeck(builder_bin_path=NELSIE_BIN, image_directory=ASSETS_DIR)
