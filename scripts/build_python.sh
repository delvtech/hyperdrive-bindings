#!/bin/bash

# hyperdrivepy install for Linux-based platforms

echo "install required packages for building wheels"
python -m pip install --upgrade -r requirements-dev.txt
python -m pip install auditwheel

echo "nav into the crate so relative paths work"
cd crates/hyperdrivepy

echo "build the wheel for the current platform"
python setup.py bdist_wheel

echo "repair built wheel to be manylinux into wheelhouse"
# wheelhouse is in the package root
mkdir ../../wheelhouse 
# Make the image manylinux complient
auditwheel repair dist/hyperdrivepy-*-linux_x86_64.whl -w ../../wheelhouse --plat manylinux_2_34_x86_64