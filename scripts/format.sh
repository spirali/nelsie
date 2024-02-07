#!/bin/bash

# Formats and lints files in-place, or just checks them for formatting

DIRS="tests python"

set -e

python -m ruff format --check $DIRS
python -m ruff check $DIRS