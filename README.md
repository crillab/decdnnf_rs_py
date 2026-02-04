# decdnnf-py

Python bindings for [decdnnf-rs](https://crates.io/crates/decdnnf_rs/1.1.0), built using [PyO3](https://github.com/PyO3/pyo3).

## How to build

Compile with [maturin](https://pypi.org/project/maturin/).

You can install `maturin` in a virtual environment:

```
me@machine:~/decdnnf-py$ python3.12 -m venv venv
me@machine:~/decdnnf-py$ source venv/bin/activate
(venv) me@machine:~/decdnnf-py$ pip install maturin[patchelf]
[...]
Successfully installed maturin-1.11.5 patchelf-0.17.2.4
```

You can then create the Python module and install e.g. in the current virtual environment:

```
(venv) me@machine:~/decdnnf-py$ maturin develop
[...]
🛠 Installed decdnnf-rs-py-0.1.0
```

## How to use

See the `examples` directory.

## License

Decdnnf-py is developed at CRIL (Univ. Artois & CNRS).
It is made available under the terms of the GNU GPLv3 license.
