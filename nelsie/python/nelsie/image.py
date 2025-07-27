import os.path
from dataclasses import dataclass
from typing import Literal

from .steps import Sv, Sn, Step, get_step

ImageFormat = Literal["png", "svg", "jpeg", "ora"]
PathOrImageData = str | tuple[bytes | str, ImageFormat]


@dataclass
class RawPathImage:
    path: str


@dataclass
class RawMemImage:
    data_id: int
    format: ImageFormat


known_suffixes = (".png", ".svg", ".jpg", ".jpeg", ".ora")
known_formats = ("png", "svg", "jpeg", "ora")


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


@dataclass
class ImageContent:
    path_or_data: Sn[str | tuple[int, str]]
    enable_steps: Sv[bool]
    shift_steps: Sv[int]

    def to_raw(self, step: Step, ctx) -> RawPathImage | RawMemImage | None:
        path_or_data = get_step(self.path_or_data, step)
        if path_or_data is None:
            return None
        if isinstance(path_or_data, tuple):
            data, data_type = path_or_data
            data_id = id(data)
            ctx.shared_data[data_id] = data
            return RawMemImage(data_id, data_type)
        return RawPathImage(path_or_data)
