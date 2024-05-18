from testutils import change_workdir
import subprocess
import sys
import time
import hashlib
import shutil
import os
from conftest import ASSETS_DIR


def test_watch(tmp_path):
    def copy_image(name):
        shutil.copy(str(os.path.join(ASSETS_DIR, name)), str(tmp_path.joinpath("test.jpeg")))

    def write(content):
        tmp_path.joinpath("slides.py").write_text(
            f"""
from nelsie import SlideDeck

deck = SlideDeck()

@deck.slide()
def test(slide):
    {content}

deck.render("out.pdf")"""
        )

    def make_hash():
        with open("out.pdf", "rb") as f:
            return hashlib.md5(f.read())

    with change_workdir(tmp_path):
        copy_image("testimg.jpeg")
        write("slide.box(width=100, height=100, bg_color='green')")
        p = subprocess.Popen([sys.executable, "-m", "nelsie", "watch", "slides.py"])
        time.sleep(1.0)

        hash1 = make_hash()
        assert p.poll() is None

        copy_image("testimg.png")
        time.sleep(1.0)
        hash2 = make_hash()
        assert hash1.hexdigest() == hash2.hexdigest()

        write("slide.image('test.jpeg', width=100, height=100)")
        time.sleep(1.0)
        hash3 = make_hash()
        assert hash1.hexdigest() != hash3.hexdigest()

        copy_image("testimg.jpeg")
        time.sleep(1.0)
        hash4 = make_hash()
        assert hash3.hexdigest() != hash4.hexdigest()

        assert p.poll() is None
        p.kill()
        p.wait()
