import contextlib
import os
import sys

import pytest
import fitz
from conftest import CHECKS_DIR
from PIL import Image, ImageChops, ImageStat

DEFAULT_THRESHOLD = 105.0


@contextlib.contextmanager
def change_workdir(workdir):
    cwd = os.getcwd()
    try:
        os.chdir(workdir)
        yield
    finally:
        os.chdir(cwd)


def concat_images(imgs):
    dst = Image.new("RGBA", (sum([im.width for im in imgs]), imgs[0].height))
    x = 0
    for im in imgs:
        dst.paste(im, (x, 0))
        x += im.width
    return dst


def compare_images(new_dir, old_dir, n_slides, threshold, resize=False):
    new_names = os.listdir(new_dir)
    new_names.sort()
    if len(new_names) != n_slides:
        raise Exception(f"Expected to produce {n_slides} but {len(new_names)} produced")
    try:
        old_names = os.listdir(old_dir)
    except FileNotFoundError:
        raise Exception(f"Checks {old_dir} does not exists")
    old_names.sort()
    if new_names != old_names:
        raise Exception(f"Produced files do not match with check files; new = {new_names}, old = {old_names}")
    for name1, name2 in zip(new_names, old_names):
        new_img = Image.open(os.path.join(new_dir, name1))
        old_img = Image.open(os.path.join(old_dir, name2))
        if resize:
            new_img = new_img.resize(old_img.size)
            new_img = new_img.convert(old_img.mode)
        difference = ImageChops.difference(new_img, old_img)
        stat = ImageStat.Stat(difference)
        diff = sum(stat.sum) / 255.0
        if diff > threshold:
            combined = concat_images([new_img, old_img, difference])
            path = os.path.abspath(f"combined-{name1}.png")
            combined.save(path)
            raise Exception(f"Slide {os.path.join(new_dir, name1)} difference is {diff} (limit is {threshold})")


def check(
    n_slides: int = 1,
    error=None,
    error_match: str | None = None,
    windows_threshold=None,
    deck_kwargs=None,
):
    def wrapper(fn):
        name = fn.__name__
        if name.startswith("test_"):
            name = name[5:]

        def helper(tmp_path, deck_builder):
            with change_workdir(tmp_path):
                if deck_kwargs is None:
                    deck = deck_builder()
                else:
                    deck = deck_builder(**deck_kwargs)
                fn(deck)
                if error is not None:
                    with pytest.raises(error, match=error_match):
                        deck.render("png", "png")
                else:
                    deck.render("png", "png")
                    with open(os.path.join(tmp_path, "check.txt"), "w") as f:
                        f.write(name)
                    deck.render(os.path.join(tmp_path, "output.pdf"), "pdf")
                    os.mkdir("pdf2png")

                    doc = fitz.open("output.pdf")
                    for i, page in enumerate(doc.pages()):
                        pixmap = page.get_pixmap()
                        img = pixmap.tobytes()
                        with open(os.path.join("pdf2png", f"{i}.png"), "wb") as f:
                            f.write(img)
                    threshold = DEFAULT_THRESHOLD
                    if windows_threshold is not None and sys.platform == "win32":
                        threshold = windows_threshold
                    compare_images(
                        os.path.join(tmp_path, "png"),
                        os.path.join(CHECKS_DIR, "png", name),
                        n_slides,
                        threshold=threshold,
                    )
                    compare_images(
                        os.path.join(tmp_path, "pdf2png"),
                        os.path.join(CHECKS_DIR, "pdf2png", name),
                        n_slides,
                        threshold=threshold,
                    )

        return helper

    return wrapper
