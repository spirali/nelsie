from typing import Optional
import json
import subprocess


def render_slides(
    nelsie_bin: str,
    root: dict,
    output_pdf: Optional[str],
    output_svg: Optional[str],
    output_png: Optional[str],
    debug: bool = False,
):
    data = json.dumps(root, indent=2)
    print(data)
    args = [nelsie_bin]
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
    stdout, _stderr = p.communicate(data.encode())
    print(stdout.decode())
    if p.returncode != 0:
        stdout = stdout.decode()
        raise Exception("Rendering failed:\n" + stdout)
