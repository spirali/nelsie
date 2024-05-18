import argparse
import importlib
import traceback
from . import nelsie as nelsie_rs
from . import watch


def parse_args():
    parser = argparse.ArgumentParser()
    subparser = parser.add_subparsers()
    watch_parser = subparser.add_parser("watch")
    watch_parser.add_argument("source_filename")
    return parser.parse_args()


def load_py_file(filename):
    watch._WATCH_SET = set()
    importlib.machinery.SourceFileLoader("slides", filename).load_module()
    s = watch._WATCH_SET
    watch._WATCH_SET = None
    s.add(filename)
    return list(s)


def main():
    args = parse_args()
    source_filename = args.source_filename
    print("Initial build", source_filename)
    files = load_py_file(source_filename)
    print("Watching", source_filename)
    while True:
        nelsie_rs.watch(files)
        print("Building ...", source_filename)
        try:
            files = load_py_file(source_filename)
            print("... finished")
        except Exception:
            traceback.print_exc()


if __name__ == "__main__":
    main()
