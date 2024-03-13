[![license: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-lightgrey)](http://www.apache.org/licenses/LICENSE-2.0)
[![DELV - Terms of Service](https://img.shields.io/badge/DELV-Terms_of_Service-orange)](https://elementfi.s3.us-east-2.amazonaws.com/element-finance-terms-of-service.pdf)

# Hyperdrive-SDK

[Hyperdrive](https://hyperdrive.delv.tech) is an automated market maker that enables fixed-rate markets to be built on top of arbitrary yield sources.

This repo contains hyperdrivepy and hyperdrive-wasm, which are Rust-powered Python and WASM packages for simulating the Hyperdrive AMM.

## Hyperdrivepy local install

The `hyperdrivepy` can be installed quickly via `pip install hyperdrivepy`.

For a local installation, this repo must include a simulation link to the hyperdrive-rust source code.
From the `hyperdrive-sdk` project root, run:

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
These are built from the Hyperdrive abi json with pypechain.
From the hyperdrive-sdk project root, run:

```shell
pip install --upgrade -r requirements-dev.txt
pypechain --line-length 120 --output-dir crates/hyperdrivepy/python/hyperdrivepy/pypechain_types hyperdrive/out/IERC4626Hyperdrive.sol/
```

## Disclaimer

This project is a work-in-progress.
The language used in this code and documentation is not intended to, and does not, have any particular financial, legal, or regulatory significance.

---

Copyright © 2024  DELV

Licensed under the Apache License, Version 2.0 (the "OSS License").

By accessing or using this code, you signify that you have read, understand and agree to be bound by and to comply with the [OSS License](http://www.apache.org/licenses/LICENSE-2.0) and [DELV's Terms of Service](https://elementfi.s3.us-east-2.amazonaws.com/element-finance-terms-of-service.pdf). If you do not agree to those terms, you are prohibited from accessing or using this code.

Unless required by applicable law or agreed to in writing, software distributed under the OSS License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the OSS License and the DELV Terms of Service for the specific language governing permissions and limitations.
