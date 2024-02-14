# Installation

Nelsie requires Python 3.10+.

## Installation via `pip` (recommended)


```commandline
$ pip install nelsie
```

Nelsie supports Linux, Windows, and MacOS X on all major platforms.


## Installation from sources

* Install Rust (https://rustup.rs/)
* Install [Maturin](https://www.maturin.rs/) (`pip install maturin`)
* Run in Nelsie source code directory:
  ```commandline
  $ python3 -m venv venv
  $ source venv/bin/activate
  $ maturin build --release
  ```