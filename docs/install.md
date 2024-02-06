# Installation

```commandline
$ pip install nelsie
```

# Installation from sources

* Install Rust (https://rustup.rs/)
* Install [Maturin](https://www.maturin.rs/) (`pip install maturin`)
* Run in Nelsie source code directory:
  ```commandline
  python3 -m venv venv
  source venv/bin/activate
  maturin build --release
  ```