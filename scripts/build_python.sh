#!/bin/bash

# hyperdrivepy install for Linux-based platforms
# This is a temporary script until we finish seting up cibuildwheel for linux
# The script assumes that the `hyperdrive` repo is cloned inside this repository root


echo "install required packages for building wheels"
python -m pip install --upgrade -r requirements-dev.txt
python -m pip install auditwheel
python -m pip install maturin

echo "nav into the crate so relative paths work"
cd crates/hyperdrivepy

echo "build the wheel for the current platform"
# python setup.py bdist_wheel
# python -m maturin build

# echo "repair built wheel to be manylinux into wheelhouse"
# # wheelhouse is in the package root
# mkdir ../../wheelhouse 
# auditwheel repair dist/hyperdrivepy-*-linux_x86_64.whl -w ../../wheelhouse --plat manylinux_2_28_x86_64