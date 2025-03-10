import os
import sys

import pytest
import shutil

PYTEST_DIR = os.path.dirname(os.path.abspath(__file__))
ROOT_DIR = os.path.dirname(PYTEST_DIR)
NELSIE_BIN = os.path.join(ROOT_DIR, "nelsie-builder", "target", "debug", "nelsie-builder")
CHECKS_DIR = os.path.join(PYTEST_DIR, "checks")
CURRENT_DIR = os.path.join(PYTEST_DIR, "current")
ASSETS_DIR = os.path.join(PYTEST_DIR, "assets")

sys.path.insert(0, os.path.join(ROOT_DIR, "nelsie-api"))

from nelsie import Resources, SlideDeck  # noqa


def new_resources(**kwargs):
    resources = Resources(**kwargs)
    return resources


@pytest.fixture(scope="session")
def resources():
    return new_resources()


@pytest.fixture()
def deck_builder(resources):
    def helper(**kwargs):
        kwargs.setdefault("image_directory", ASSETS_DIR)
        kwargs.setdefault("resources", resources)
        return SlideDeck(**kwargs)

    return helper


@pytest.fixture()
def deck(deck_builder):
    return deck_builder()


if os.path.isdir(CURRENT_DIR):
    shutil.rmtree(CURRENT_DIR)
os.mkdir(CURRENT_DIR)
