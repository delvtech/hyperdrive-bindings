# Hyperdrive-SDK

Hyperdrive is an automated market maker that enables fixed-rate markets to be built on top of arbitrary yield sources.

This repo contains hyperdrivepy and hyperdrive-wasm, which are Rust-powered Python and WASM packages for simulating the Hyperdrive AMM.

## Disclaimer

The language used in this codebase is for coding convenience only, and is not
intended to, and does not, have any particular legal or regulatory significance.

## Install

This repo must include a simulation link to the hyperdrive-rust source code. From the `hyperdrive-sdk` project root, run:

```shell
git clone git@github.com:delvtech/hyperdrive.git ../hyperdrive
ln -s ../hyperdrive hyperdrive
```

To install the Python package `hyperdrivepy`, which wraps `hyperdrive-rs`, you need to:

- setup a [Python venv](https://docs.python.org/3/library/venv.html) that is running `Python 3.10`
- from inside the environment, run `pip install crates/hyperdrivepy`
- test the installation by running `pip install --upgrade -r requirements-dev.txt && pytest`

[optional] To build the package wheel locally, you can navigate to the package folder and use `setup.py`:

- `cd crates/hyperdrivepy`
- `python setup.py bdist_wheel`
  This will make the distribution ready (e.g. a tar.gz file and a .whl file in the dist directory) for your platform.
  To build for more platforms, we use [cbuildwheel](https://cibuildwheel.readthedocs.io/en/stable/) in our GitHub CI.

## Build Types

PoolInfo and PoolConfig are passed into many of the functions.
These are built from the hyperdrive abi json with pypechain.
From the hyperdrive-sdk project root, run:

```shell
pip install --upgrade -r requirements-dev.txt
pypechain hyperdrive/out/IHyperdrive.sol/IHyperdrive.json --output_dir crates/hyperdrive-sdk/python/hyperdrivepy/pypechain_types
```
