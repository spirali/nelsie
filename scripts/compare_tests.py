import shutil
from dataclasses import dataclass
from pathlib import Path
import os
import click
from PIL import Image, ImageChops, ImageStat

ROOT = os.path.abspath(os.path.dirname(os.path.dirname(__file__)))
CHECKS = os.path.join(ROOT, "tests", "checks")

LIMIT = 0.000001


def image_diff(path1, path2) -> float | None:
    try:
        new_img = Image.open(path1)
        old_img = Image.open(path2)
    except FileNotFoundError:
        return float("inf")
    difference = ImageChops.difference(new_img, old_img)
    stat = ImageStat.Stat(difference)
    return sum(stat.sum)


@dataclass
class FailedTest:
    name: str
    files: list[str]
    diffs: list[float]
    test_dir: str
    check_dir: str


def collect_failed_tests() -> list[FailedTest]:
    paths = Path(f"/tmp/pytest-of-{os.getlogin()}/pytest-current/").rglob(
        "*current/check.txt"
    )
    fails = []
    for path in paths:
        with open(path, "r") as f:
            name = f.read()
        test_dir = os.path.join(os.path.dirname(path), name)
        check_dir = os.path.join(CHECKS, name)
        files = set()
        files.update(os.listdir(test_dir))
        if os.path.exists(check_dir):
            files.update(os.listdir(check_dir))
        files = sorted(files)
        diffs = []
        for file in files:
            diff = image_diff(
                os.path.join(test_dir, file), os.path.join(check_dir, file)
            )
            diffs.append(diff)
        if max(diffs) > LIMIT:
            fails.append(
                FailedTest(
                    name=name,
                    files=files,
                    diffs=diffs,
                    test_dir=test_dir,
                    check_dir=check_dir,
                )
            )
    return fails


def report(failed_tests):
    filename = "report.html"
    img_dir = os.path.abspath("report_imgs")
    style = "border: 3px solid black"
    shutil.rmtree(img_dir, ignore_errors=True)
    os.mkdir(img_dir)
    for failed_test in failed_tests:
        shutil.copytree(
            os.path.join(failed_test.test_dir),
            os.path.join("report_imgs", failed_test.name),
        )
    with open(filename, "w") as f:
        f.write("<html><body><h1>Test report</h1>")
        for failed_test in failed_tests:
            f.write(f"<h2>{failed_test.name}</h2>")
            f.write("<table>")
            f.write(
                "<tr><td>Name</td><td>Reference</td><td>Test</td><td>Diff</td></tr>"
            )
            for file, df in zip(failed_test.files, failed_test.diffs):
                f.write(f"<tr><td>{file}</td>")
                f.write(
                    f"<td><img style='{style}' width='400' src='{os.path.join(failed_test.check_dir, file)}'/></td>"
                )
                f.write(
                    f"<td><img style='{style}' width='400' src='{os.path.join(img_dir, failed_test.name, file)}'/></td>"
                )
                f.write(f"<td>{df}</td>")
                f.write("</tr>")
            f.write("</table>")
        f.write("</body></html>")
    print(f"Report written into '{filename}'")


def update(failed_tests):
    for failed_test in failed_tests:
        print(f"Updating {failed_test.name}")
        shutil.rmtree(failed_test.check_dir, ignore_errors=True)
        shutil.copytree(
            os.path.join(failed_test.test_dir),
            os.path.join(failed_test.check_dir),
        )


@click.command()
@click.option("--do-update/--do-not-update", default=False)
def main(do_update):
    failed_tests = collect_failed_tests()
    if failed_tests:
        if not do_update:
            print("Failed tests:")
            for failed_test in failed_tests:
                print(failed_test.name)
            report(failed_tests)
        else:
            update(failed_tests)
    else:
        print("All tests are ok")


if __name__ == "__main__":
    main()
