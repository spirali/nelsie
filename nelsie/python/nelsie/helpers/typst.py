import subprocess
import hashlib
import os


class Typst:

    def __init__(self, typst_path="typst", cache_path="./typst_cache"):
        cache_path = os.path.abspath(cache_path)
        if not os.path.isdir(cache_path):
            os.makedirs(cache_path)
        self.typst_path = typst_path
        self.cache_path = cache_path

        self.default_header = "#set page(width: auto, height: auto, margin: (x: 0pt, y: 0pt))"
        self.version = self._call_typst(["--version"])
        hasher = hashlib.sha1()
        hasher.update(self.version)
        self.hasher = hasher

    def _call_typst(self, args: list[str]):
        r = subprocess.run([self.typst_path, *args], check=True, stdout=subprocess.PIPE)
        return r.stdout

    def get_path(self, text: str, use_header: bool = True):
        if use_header:
            text = f"{self.default_header}\n{text}"
        hasher = self.hasher.copy()
        hasher.update(text.encode())
        output_path = os.path.join(self.cache_path, f"{hasher.digest().hex()}.svg")
        if os.path.isfile(output_path):
            return output_path
        input_path = os.path.join(self.cache_path, f"{hasher.digest().hex()}.typ")
        with open(input_path, "w") as f:
            f.write(text)
        self._call_typst(["compile", "--format=svg", input_path])
        return output_path

    def render(self, slide, text: str, use_header: bool = True, **kwargs):
        slide.image(self.get_path(text, use_header), **kwargs)
