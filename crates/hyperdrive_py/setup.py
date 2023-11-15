"""Entry point for installing hyperdrive math python package"""
from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="hyperdrive_py",
    version="0.3.0",
    packages=["hyperdrive_py"],
    package_dir={"": "python"},
    rust_extensions=[
        RustExtension("hyperdrive_py.hyperdrive_py", binding=Binding.PyO3),
    ],
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False,
    python_requires=">=3.8",
)
