import json
import subprocess
from typing import Optional


def render_slides(
    builder_bin_path: str,
    root: dict,
    output_pdf: Optional[str],
    output_svg: Optional[str],
    output_png: Optional[str],
    debug: bool = False,
):
    data = json.dumps(root, indent=2 if debug else None)
    if debug:
        print(data)
    args = [builder_bin_path]
    if debug:
        args.append("--debug")
    if output_pdf:
        args += ["--output-pdf", output_pdf]
    if output_svg:
        args += ["--output-svg", output_svg]
    if output_png:
        args += ["--output-png", output_png]

    p = subprocess.Popen(
        args, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, stdin=subprocess.PIPE
    )
    print("Building slides ...")
    stdout, _stderr = p.communicate(data.encode())
    print(stdout.decode())
    if p.returncode != 0:
        stdout = stdout.decode()
        raise Exception("Rendering failed:\n" + stdout)
    print("... done")