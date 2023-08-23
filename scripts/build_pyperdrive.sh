#!/bin/bash

# Pyperdrive install for Linux-based platforms
# This is a temporary script until we finish seting up cibuildwheel for linux
# The script assumes that the `hyperdrive` repo is cloned inside this repository root


echo "install required packages for building wheels"
python -m pip install --upgrade -r requirements-dev.txt

echo "nav into the crate so relative paths work"
cd crates/pyperdrive

echo "build the wheel for the current platform"
python setup.py bdist_wheel

echo "copy built wheel files from pyperdrive into wheelhouse"
# wheelhouse is in the package root
mkdir ../../wheelhouse 
cp dist/* ../../wheelhouse/
