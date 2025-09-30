from . import nelsie as nelsie_rs
from .steps import StepVal

MODES = ("e", "n", "en")


def process_step_line(line: str, delimiter: str, prev_steps, prev_add_empty):
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

    step_def = step_def.strip()
    if step_def == "":
        return line, prev_steps, prev_add_empty

    steps, named_steps = nelsie_rs.parse_bool_steps(step_def)
    step_val = StepVal(init_values=steps, named_steps=named_steps)
    if "n" in mode:
        step_val.bool_inverse()
    return line, step_val, "e" in mode


def text_step_parser(text: str, delimiter: str):
    steps = set()
    named_steps = set()
    lines = []
    prev_step_val = None
    prev_add_empty = False
    for line in text.split("\n"):
        line_data = process_step_line(line, delimiter, prev_step_val, prev_add_empty)
        if line_data[1] is not None:
            prev_step_val = line_data[1]
            prev_add_empty = line_data[2]
            steps.update(line_data[1].values)
            named_steps.update(line_data[1].named_steps)
        lines.append(line_data)

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
