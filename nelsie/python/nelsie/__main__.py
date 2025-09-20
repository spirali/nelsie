import argparse
import importlib
import sys
import traceback
from typing import List, Set

from . import nelsie as nelsie_rs
from . import watch


def parse_args():
    parser = argparse.ArgumentParser()
    subparser = parser.add_subparsers()
    subparser.required = True
    watch_parser = subparser.add_parser("watch")
    watch_parser.add_argument("source_filename", nargs="+")
    return parser.parse_args()


def get_system_modules() -> Set[str]:
    from sys import modules
    import importlib  # noqa

    return set(modules.keys())


def reload_and_get_watched_files(
    filenames: List[str], system_modules: Set[str]
) -> List[str]:
    """
    Executes the first passed Python script from the `filenames` list
    and gathers all files that should be watched.
    """
    watch._WATCH_SET = set()

    # Try to reload non-system modules, to refresh Python imports from the main file
    # We need to skip some system modules, otherwise reloading them could break Python
    for name, module in dict(sys.modules).items():
        if (
            name in sys.builtin_module_names
            or name in system_modules
            or name.startswith("nelsie.")
        ):
            continue
        try:
            importlib.reload(module)
        except BaseException:
            pass

    # Re-execute the file
    importlib.machinery.SourceFileLoader("slides", filenames[0]).load_module()

    s = watch._WATCH_SET
    watch._WATCH_SET = None

    to_watch = list(filenames)
    to_watch.extend(sorted(s))
    return to_watch


def main():
    args = parse_args()
    source_filenames = args.source_filename

    system_modules = get_system_modules()

    print("Initial build of", source_filenames[0])
    files = reload_and_get_watched_files(source_filenames, system_modules)
    print("Watching", files)
    while True:
        nelsie_rs.watch(files)
        print("Building ...", source_filenames[0])
        try:
            files = reload_and_get_watched_files(source_filenames, system_modules)
            print("... finished")
        except BaseException:
            traceback.print_exc()


if __name__ == "__main__":
    main()
