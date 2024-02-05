from setuptools import setup

setup(
    name="mkdocs-nelsie-plugin",
    version="0.0.1",
    description="MkDocs plugin for building Nelsie examples in documentation",
    author="Ada Böhm",
    author_email="ada@kreatrix.org",
    url="https://github.com/spirali/nelsie",
    packages=["mkdocs_nelsie_plugin"],
    entry_points={
        "mkdocs.plugins": [
            "nelsie = mkdocs_nelsie_plugin:NelsiePlugin",
        ]
    },
)
