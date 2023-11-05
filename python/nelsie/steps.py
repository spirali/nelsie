def to_step_value(obj, parser=None):
    return {"const": parser(obj)}
