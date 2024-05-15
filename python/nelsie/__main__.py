import argparse
import importlib
import traceback
from . import nelsie as nelsie_rs


def parse_args():
    parser = argparse.ArgumentParser()
    subparser = parser.add_subparsers()
    watch_parser = subparser.add_parser("watch")
    watch_parser.add_argument("source_filename")
    return parser.parse_args()


def load_py_file(filename):
    importlib.machinery.SourceFileLoader("slides", filename).load_module()


def main():
    args = parse_args()
    source_filename = args.source_filename
    print("Initial build", source_filename)
    load_py_file(source_filename)
    print("Watching", source_filename)
    while True:
        nelsie_rs.watch(source_filename)
        print("Building ...", source_filename)
        try:
            load_py_file(source_filename)
            print("... finished")
        except Exception:
            traceback.print_exc()


if __name__ == "__main__":
    main()
