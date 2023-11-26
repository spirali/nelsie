#!/usr/bin/env python

import sys

from setuptools import find_packages, setup

if sys.version_info.major < 3 or (
    sys.version_info.major == 3 and sys.version_info.minor < 9
):
    sys.exit("Python 3.9 or newer is required")

VERSION: str | None = None
with open("nelsie/version.py") as f:
    exec(f.read())
if not isinstance(VERSION, str):
    raise Exception("version.py executed but VERSION was not set properly")

try:
    from wheel.bdist_wheel import bdist_wheel as _bdist_wheel

    class bdist_wheel(_bdist_wheel):
        def finalize_options(self):
            _bdist_wheel.finalize_options(self)
            # Mark us as not a pure python package
            self.root_is_pure = False

        def get_tag(self):
            python, abi, plat = _bdist_wheel.get_tag(self)
            # We don't contain any python bindings
            python, abi = "py3", "none"
            return python, abi, plat

except ImportError:
    bdist_wheel = None

setup(
    name="nelsie",
    version=VERSION,
    description="Framework for creating slides",
    long_description="""
Nelsie is a Framework for creating slides in Python,
      """,
    author="Ada Böhm",
    author_email="ada@kreatrix.org",
    url="https://github.com/spirali/nelsie",
    packages=find_packages(),
    package_data={'': ['nelsie-builder']},
    install_requires=[],
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: MIT License",
    ],
    cmdclass={
        "bdist_wheel": bdist_wheel,
    },
)
