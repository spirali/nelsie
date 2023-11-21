import contextlib
import os
from PIL import Image, ImageChops, ImageStat
from conftest import NELSIE_BIN, CHECKS_DIR
import pytest


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


def compare_images(new_dir, old_dir, n_slides, threshold):
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
        raise Exception(
            f"Produced files do not match with check files; new = {new_names}, old = {old_names}"
        )
    for name in new_names:
        new_img = Image.open(os.path.join(new_dir, name))
        old_img = Image.open(os.path.join(old_dir, name))
        difference = ImageChops.difference(new_img, old_img)
        stat = ImageStat.Stat(difference)
        diff = sum(stat.sum)
        if diff > threshold:
            combined = concat_images([new_img, old_img, difference])
            path = os.path.abspath("combined.png")
            combined.save(path)
            raise Exception(
                f"Slide {os.path.join(new_dir, name)} difference is {diff} (limit is {threshold})"
            )


def check(n_slides: int = 1, error=None, error_match: str | None = None):
    def wrapper(fn):
        name = fn.__name__
        if name.startswith("test_"):
            name = name[5:]

        def helper(tmp_path, deck, *args, **kwargs):
            with change_workdir(tmp_path):
                fn(deck)
                if error is not None:
                    with pytest.raises(error, match=error_match):
                        deck.render(output_png=name)
                else:
                    deck.render(output_png=name)
                    compare_images(
                        os.path.join(tmp_path, name),
                        os.path.join(CHECKS_DIR, name),
                        n_slides,
                        threshold=0.001,
                    )

        return helper

    return wrapper
