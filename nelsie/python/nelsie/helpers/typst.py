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

        self.default_header = self.get_header()
        self.version = self._call_typst(["--version"])
        hasher = hashlib.sha1()
        hasher.update(self.version)
        self.hasher = hasher
        self.generated = set()

    def _call_typst(self, args: list[str]):
        r = subprocess.run([self.typst_path, *args], check=True, stdout=subprocess.PIPE)
        return r.stdout

    def get_header(self, width="auto", height="auto", margin_x="0pt", margin_y="0pt"):
        return f"#set page(width: {width}, height: {height}, margin: (x: {margin_x}, y: {margin_y}))\n"

    def get_path(self, text: str, use_header: bool = True):
        if use_header:
            text = f"{self.default_header}\n{text}"
        hasher = self.hasher.copy()
        hasher.update(text.encode())
        hex = hasher.digest().hex()
        self.generated.add(hex)
        output_path = os.path.join(self.cache_path, f"{hex}.svg")
        if os.path.isfile(output_path):
            return output_path
        input_path = os.path.join(self.cache_path, f"{hex}.typ")
        with open(input_path, "w") as f:
            f.write(text)
        self._call_typst(["compile", "--format=svg", input_path])
        return output_path

    def render(self, slide, text: str, use_header: bool = True, **kwargs):
        slide.image(self.get_path(text, use_header), **kwargs)

    def clean_cache(self):
        for path in os.listdir(self.cache_path):
            if not path.endswith(".typ") and not path.endswith(".svg"):
                continue
            base = path[:-4]
            if base not in self.generated:
                os.unlink(os.path.join(self.cache_path, path))
