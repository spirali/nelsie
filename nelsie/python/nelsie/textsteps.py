from . import nelsie as nelsie_rs
from .steps import StepVal

MODES = ("e", "n", "en")


def process_step_line(line: str, delimiter: str):
    s = line.rsplit(delimiter, 1)
    if len(s) == 1:
        return line, None, False
    line, rest = s
    if ";" in rest:
        mode, step_def = rest.split(";", 1)
        mode = mode.strip()
        if mode not in MODES:
            raise Exception(f"Invalid mode '{mode}'.")
    else:
        mode = ""
        step_def = rest
    steps, named_steps = nelsie_rs.parse_bool_steps(step_def)
    step_val = StepVal(init_values=steps, named_steps=named_steps)
    if "n" in mode:
        step_val.bool_inverse()
    return line, step_val, "e" in mode


def text_step_parser(text: str, delimiter: str):
    steps = set()
    named_steps = set()
    lines = [process_step_line(line, delimiter) for line in text.split("\n")]
    for line_data in lines:
        if line_data[1] is not None:
            steps.update(line_data[1].values)
            named_steps.update(line_data[1].named_steps)
    result = {}
    steps.add(1)
    for step in steps:
        current_lines = []
        for line, step_val, add_empty in lines:
            if step_val is None or step_val.get_step(step):
                current_lines.append(line)
            elif add_empty:
                current_lines.append("")
        result[step] = "\n".join(current_lines)
    return StepVal(init_values=result, named_steps=list(named_steps))
