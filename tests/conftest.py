import os

import pytest
import shutil

PYTEST_DIR = os.path.dirname(os.path.abspath(__file__))
ROOT_DIR = os.path.dirname(PYTEST_DIR)
CHECKS_DIR = os.path.join(PYTEST_DIR, "checks")
CURRENT_DIR = os.path.join(PYTEST_DIR, "current")
ASSETS_DIR = os.path.join(PYTEST_DIR, "assets")

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
        kwargs.setdefault("resources", resources)
        return SlideDeck(**kwargs)

    return helper


@pytest.fixture()
def deck(deck_builder):
    return deck_builder()


if os.path.isdir(CURRENT_DIR):
    shutil.rmtree(CURRENT_DIR)
os.mkdir(CURRENT_DIR)
