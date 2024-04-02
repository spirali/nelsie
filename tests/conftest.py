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


def new_resources(**kwargs):
    resources = Resources(**kwargs)

    # Let us fix the fonts, so we have the same results across all OSs
    resources.load_fonts_dir(os.path.join(ASSETS_DIR, "fonts"))
    return resources


@pytest.fixture(scope="session")
def resources():
    return new_resources()


@pytest.fixture()
def deck_builder(resources):
    def helper(**kwargs):
        kwargs.setdefault("image_directory", ASSETS_DIR)
        kwargs.setdefault("default_font", "DejaVu Sans")
        kwargs.setdefault("default_monospace_font", "DejaVu Sans Mono")
        kwargs.setdefault("resources", resources)
        return SlideDeck(**kwargs)

    return helper


@pytest.fixture()
def deck(deck_builder):
    return deck_builder()
