"""Entry point for installing hyperdrive math python package"""
from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="pyperdrive",
    version="0.1.0",
    packages=["pyperdrive"],
    package_dir={"": "python"},
    rust_extensions=[
        RustExtension("pyperdrive.pyperdrive", binding=Binding.PyO3),
    ],
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False,
    python_requires=">=3.8",
)
