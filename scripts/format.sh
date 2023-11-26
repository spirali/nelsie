#!/bin/bash

# Formats and lints files in-place, or just checks them for formatting

DIRS="tests nelsie-api"

set -e

if [[ "$1" != "--check" && "x$1" != "x" ]]; then
    echo "Run as '$0 --check' or just '$0'"
    exit 1
fi

cd `dirname $0`/..

if [[ "$1" == "--check" ]]; then
    python -m isort --check-only --profile black $DIRS
    python -m black --check $DIRS
    # Lint Python code
    # By default, this is skipped in checking
    #poetry run flake8 tests interlab
    echo "All code checks completed successfully"
else
    # Format Python code
    python -m isort --profile black $DIRS
    python -m black $DIRS
    # Lint Python code
    python -m flake8 $DIRS
fi

