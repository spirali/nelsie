import os.path
from dataclasses import dataclass
from typing import Literal

from .steps import Sv, Sn, Step, get_step, sn_apply
from . import nelsie as nelsie_rs

ImageFormat = Literal["png", "svg", "jpeg", "ora"]
PathOrImageData = str | tuple[bytes | str, ImageFormat]

known_suffixes = (".png", ".svg", ".jpg", ".jpeg", ".ora")
known_formats = ("png", "svg", "jpeg", "ora")


class RawImage:
    """Dummy type for nelsie.image.RawImage"""

    pass


def check_image_path_or_data(obj):
    if isinstance(obj, str):
        if not os.path.isfile(obj):
            if not os.path.exists(obj):
                raise Exception(f"Path '{obj}' does not exists")
            else:
                raise Exception(f"Path '{obj}' exists but it is not a file")
        obj = obj.lower()
        for suffix in known_suffixes:
            if obj.endswith(suffix):
                return
        raise Exception("Unknown image format extension")
    if isinstance(obj, tuple) and len(obj) == 2 and obj[1] in known_formats:
        if obj[1] == "svg":
            if not isinstance(obj[0], str):
                raise Exception(f"Image format '{obj[1]}' expect 'str' as data")
        else:
            if not isinstance(obj[0], bytes):
                raise Exception(f"Image format '{obj[1]}' expect 'bytes' as data")
        return  # Ok
    raise Exception("Image specification has to be path or tuple [bytes, format]")


def normalize_image_path(path):
    if isinstance(path, str):
        return os.path.abspath(path)
    return path


def _put_into_shared_data(path_or_data: PathOrImageData | None, content, shared_data, steps):
    if path_or_data is None:
        return
    check_image_path_or_data(path_or_data)
    if isinstance(path_or_data, str):
        image = nelsie_rs.load_image(path_or_data, content.enable_steps)
        steps.update(image.named_steps())
        shared_data[path_or_data] = image
        return
    data, data_type = path_or_data
    key = (id(data), data_type)
    if key not in shared_data:
        image = nelsie_rs.create_mem_image(data, data_type, content.enable_steps)
        steps.update(image.named_steps())
        shared_data[key] = image


@dataclass
class ImageContent:
    path_or_data: Sn[PathOrImageData]
    enable_steps: Sv[bool]
    shift_steps: Sv[int]

    def traverse_tree(self, shared_data, steps: set[Step]):
        sn_apply(self.path_or_data, lambda path_or_data: _put_into_shared_data(path_or_data, self, shared_data, steps))

    def to_raw(self, step: Step, ctx):
        path_or_data = get_step(self.path_or_data, step)
        if path_or_data is None:
            return None
        if isinstance(path_or_data, tuple):
            key = (id(path_or_data[0]), path_or_data[1])
        else:
            key = path_or_data
        return ctx.shared_data[key].at_step(step)
