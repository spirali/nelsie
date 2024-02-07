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


@pytest.fixture(scope="session")
def resources():
    resources = Resources()

    # Let us fix the fonts, so we have the same results across all OSs
    resources.load_fonts_dir(os.path.join(ASSETS_DIR, "fonts"))
    return resources


@pytest.fixture()
def deck(resources):
    return SlideDeck(
        image_directory=ASSETS_DIR,
        resources=resources,
        default_font="DejaVu Sans",
        default_monospace_font="DejaVu Sans Mono",
    )
