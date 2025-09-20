import os
import importlib.resources as import_resources

from . import nelsie as nelsie_rs
from . import data

BUILTIN_FONTS_DIR = os.path.abspath(import_resources.files(data) / "fonts")


class Resources:
    def __init__(
        self,
        *,
        builtin_fonts: bool = True,
        system_fonts: bool = False,
        system_fonts_for_svg: bool = True,
        default_code_syntaxes: bool = True,
        default_code_themes: bool = True,
    ):
        self._resources = nelsie_rs.Resources(
            system_fonts,
            system_fonts_for_svg,
            default_code_syntaxes,
            default_code_themes,
        )
        if builtin_fonts:
            self._resources.load_fonts_dir(BUILTIN_FONTS_DIR)
            self._resources.set_generic_family("sans-serif", "DejaVu Sans")
            self._resources.set_generic_family("monospace", "DejaVu Sans Mono")

    def set_sans_serif(self, font_name):
        self._resources.set_generic_family("sans-serif", font_name)

    def set_monospace(self, font_name):
        self._resources.set_generic_family("monospace", font_name)

    def set_serif(self, font_name):
        self._resources.set_generic_family("serif", font_name)

    def load_code_syntax_dir(self, path: str):
        self._resources.load_code_syntax_dir(path)

    def load_code_theme_dir(self, path: str):
        self._resources.load_code_theme_dir(path)

    def load_fonts_dir(self, path: str):
        self._resources.load_fonts_dir(path)

    def syntaxes(self) -> list[tuple[str, list[str]]]:
        return self._resources.syntaxes()

    def themes(self) -> list[str]:
        return self._resources.themes()
