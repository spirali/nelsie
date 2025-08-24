_WATCH_SET: None | set = None


def watch_path(path):
    if _WATCH_SET is not None:
        _WATCH_SET.add(path)
