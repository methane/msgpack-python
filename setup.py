#!/usr/bin/env python
import os
import sys

from setuptools import setup
from setuptools_rust import Binding, RustExtension

PYPY = hasattr(sys, "pypy_version_info")

rust_extensions = []
if not PYPY and not os.environ.get("MSGPACK_PUREPYTHON"):
    rust_extensions.append(
        RustExtension(
            "msgpack._cmsgpack",
            path="Cargo.toml",
            binding=Binding.PyO3,
        )
    )

setup(
    rust_extensions=rust_extensions,
    packages=["msgpack"],
    zip_safe=False,
)
