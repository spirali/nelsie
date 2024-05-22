import base64
import shutil
from dataclasses import dataclass
from io import BytesIO
from pathlib import Path
import os
import click
from PIL import Image, ImageChops, ImageStat
import numpy as np

ROOT = os.path.abspath(os.path.dirname(os.path.dirname(__file__)))
CHECKS = os.path.join(ROOT, "tests", "checks")

LIMIT = 105.0


def image_diff(path1, path2, resize):
    try:
        new_img = Image.open(path1)
        old_img = Image.open(path2)
    except FileNotFoundError:
        return float("inf"), None

    if resize:
        new_img = new_img.resize(old_img.size)
        new_img = new_img.convert(old_img.mode)
    difference = ImageChops.difference(new_img, old_img)
    stat = ImageStat.Stat(difference)
    s = sum(stat.sum) / 255.0
    if s > 0:
        sub = np.subtract(np.array(new_img).astype(np.cfloat), np.array(old_img).astype(np.cfloat))
        diff_data2 = -np.minimum(np.min(sub, axis=2), 0)
        diff_data1 = np.maximum(np.max(sub, axis=2), 0)
        diff_data3 = np.zeros_like(diff_data2)
        diff_data = np.stack([diff_data1, diff_data2, diff_data3], axis=2)
        diff_img = Image.fromarray(diff_data.astype(np.uint8), "RGB")
    else:
        diff_img = None
    return s, diff_img


@dataclass
class FailedTest:
    name: str
    files: list[str]
    diffs: list[float]
    test_dir: str
    check_dir: str
    diff_imgs: list
    test_type: str


def check_images(name, path, fails, subdir, resize=False):
    test_dir = os.path.join(os.path.dirname(path), subdir)
    check_dir = os.path.join(CHECKS, subdir, name)
    files = set()
    files.update(os.listdir(test_dir))
    if os.path.exists(check_dir):
        files.update(os.listdir(check_dir))
    files = sorted(files)
    diffs = []
    diff_imgs = []
    for file in files:
        diff, diff_img = image_diff(os.path.join(test_dir, file), os.path.join(check_dir, file), resize)
        diffs.append(diff)
        diff_imgs.append(diff_img)
    if max(diffs) > LIMIT:
        fails.append(
            FailedTest(
                name=name,
                files=files,
                diffs=diffs,
                test_dir=str(test_dir),
                check_dir=check_dir,
                diff_imgs=diff_imgs,
                test_type=subdir,
            )
        )


def collect_failed_tests(path, version) -> list[FailedTest]:
    if path is None:
        path = f"/tmp/pytest-of-{os.getlogin()}/pytest-current/"
    paths = Path(path).rglob(f"*{version}/check.txt")
    fails = []
    count = 0
    for path in paths:
        count += 1
        with open(path, "r") as f:
            name = f.read()
        check_images(name, path, fails, "png")
        check_images(name, path, fails, "pdf2png", resize=True)
    return fails, count


def html_inline_img(img):
    diff_img_png = BytesIO()
    img.save(diff_img_png, format="PNG")
    img_str = base64.b64encode(diff_img_png.getvalue())
    return f"data:image/png;base64,{img_str.decode()}"


def report(failed_tests):
    filename = "report.html"
    img_dir = os.path.abspath("report_imgs")
    style = "border: 3px solid black"
    shutil.rmtree(img_dir, ignore_errors=True)
    os.mkdir(img_dir)
    for failed_test in failed_tests:
        shutil.copytree(
            os.path.join(failed_test.test_dir),
            os.path.join("report_imgs", failed_test.test_type, failed_test.name),
        )
    with open(filename, "w") as f:
        f.write("<html><body><h1>Test report</h1>")
        for failed_test in failed_tests:
            f.write(f"<h2>{failed_test.name} ({failed_test.test_type})</h2>")
            f.write("<table>")
            f.write("<tr><td>Name</td><td>Reference</td><td>Test</td><td>Diff</td></tr>")
            for file, df, diff_img in zip(failed_test.files, failed_test.diffs, failed_test.diff_imgs):
                f.write(f"<tr><td>{file}</td>")
                f.write(
                    f"<td><img style='{style}' width='400' src='{os.path.join(failed_test.check_dir, file)}'/></td>"
                )
                f.write(
                    f"<td><img style='{style}' width='400' src='{os.path.join(img_dir, failed_test.test_type, failed_test.name, file)}'/></td>"
                )
                if diff_img is not None:
                    f.write(f"<td><img style='{style}' width='400' src='{html_inline_img(diff_img)}'/></td>")
                else:
                    f.write("<td></td>")
                f.write(f"<td>{df:0.2f}</td>")
                f.write("</tr>")
            f.write("</table>")
        f.write("</body></html>")
    print(f"Report written into '{filename}'")


def update(failed_tests):
    for failed_test in failed_tests:
        print(f"Updating {failed_test.name} ({failed_test.test_type})")
        shutil.rmtree(failed_test.check_dir, ignore_errors=True)
        shutil.copytree(
            failed_test.test_dir,
            failed_test.check_dir,
        )


@click.command()
@click.option("--do-update/--do-not-update", default=False)
@click.option("--path", default=None)
@click.option("--version", default="current")
def main(do_update, path, version):
    failed_tests, count = collect_failed_tests(path, version)
    if count == 0:
        print("No test found")
    elif failed_tests:
        if not do_update:
            print("Failed tests:")
            for failed_test in failed_tests:
                print(failed_test.test_type, failed_test.name)
            report(failed_tests)
        else:
            update(failed_tests)
    else:
        print(f"All tests ({count}) are ok")


if __name__ == "__main__":
    main()
