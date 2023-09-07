# Pyperdrive (Hyperdrive in Python)

Pyperdrive is a Rust-powered python package for simulating the Hyperdrive AMM.
Hyperdrive is an automated market maker that enables fixed-rate markets to be
built on top of arbitrary yield sources.

## Install

This repo must include a simulation link to the hyperdrive-rust source code. From the `pyperdrive` project root, run:

```shell
git clone git@github.com:delvtech/hyperdrive.git ../hyperdrive
ln -s ../hyperdrive hyperdrive
```

To install the Python package `pyperdrive`, which wraps the Rust `hyperdrive_math::State` struct, you need to:

- setup a [Python venv](https://docs.python.org/3/library/venv.html) that is running at least `Python 3.7`
- from inside the environment, run `pip install crates/pyperdrive`
- test the installation by running `pip install --upgrade -r requirements-dev.txt && pytest`

[optional] To build the package wheel locally, you can navigate to the package folder and use `setup.py`:

- `cd crates/pyperdrive/`
- `python setup.py bdist_wheel`
This will make the distribution ready (e.g. a tar.gz file and a .whl file in the dist directory) for your platform.
To build for more platforms, we use [cbuildwheel](https://cibuildwheel.readthedocs.io/en/stable/) in our GitHub CI.

## Disclaimer

The language used in this codebase is for coding convenience only, and is not
intended to, and does not, have any particular legal or regulatory significance.
