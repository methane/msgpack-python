#!/usr/bin/env python
import os
import sys

from setuptools import Extension, setup

PYPY = hasattr(sys, "pypy_version_info")
LIMITED_API = not PYPY and sys.version_info >= (3, 11)

libraries = []
macros = []
ext_modules = []

if sys.platform == "win32":
    libraries.append("ws2_32")
    macros = [("__LITTLE_ENDIAN__", "1")]

if not PYPY and not os.environ.get("MSGPACK_PUREPYTHON"):
    if LIMITED_API:
        macros += [("Py_LIMITED_API", "0x030B0000"), ("CYTHON_LIMITED_API", "1")]
    ext_modules.append(
        Extension(
            "msgpack._cmsgpack",
            sources=["msgpack/_cmsgpack.c"],
            libraries=libraries,
            include_dirs=["."],
            define_macros=macros,
            py_limited_api=LIMITED_API,
        )
    )
del libraries, macros

setup(
    ext_modules=ext_modules,
    options={"bdist_wheel": {"py_limited_api": "cp311"}} if LIMITED_API else {},
    packages=["msgpack"],
)
