from testutils import change_workdir
import subprocess
import sys
import time
import hashlib


def test_reload(tmp_path):
    def write(content):
        tmp_path.joinpath("slides.py").write_text(
            f"""
from nelsie import SlideDeck

deck = SlideDeck()

def test(slide):
    {content}

deck.render("out.pdf")"""
        )

    def make_hash():
        with open("out.pdf", "rb") as f:
            return hashlib.md5(f.read())

    with change_workdir(tmp_path):
        write("slide.box(width=100, height=100, bg_color='green')")
        p = subprocess.Popen([sys.executable, "-m", "nelsie", "watch", "slides.py"])
        time.sleep(1.0)

        hash1 = make_hash()
        assert p.poll() is None

        write("slide.box(width=100, height=100, bg_color='blue')")
        time.sleep(1.0)
        hash2 = make_hash()
        assert hash1 != hash2

        assert p.poll() is None
        p.kill()
        p.wait()
