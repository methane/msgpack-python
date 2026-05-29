use pyo3::exceptions::{PyBufferError, PyNotImplementedError, PyStopIteration};
use pyo3::prelude::*;
use pyo3::types::PyDict;

#[pyclass(module = "msgpack._cmsgpack")]
struct Packer {
    inner: Py<PyAny>,
}

#[pyclass(subclass, module = "msgpack._cmsgpack")]
struct Unpacker {
    inner: Py<PyAny>,
}

impl Unpacker {
    #[allow(clippy::too_many_arguments)]
    fn new_inner(
        py: Python<'_>,
        file_like: Option<PyObject>,
        read_size: usize,
        use_list: bool,
        raw: bool,
        timestamp: i64,
        strict_map_key: bool,
        object_hook: Option<PyObject>,
        object_pairs_hook: Option<PyObject>,
        list_hook: Option<PyObject>,
        unicode_errors: Option<&str>,
        max_buffer_size: usize,
        ext_hook: Option<PyObject>,
        max_str_len: isize,
        max_bin_len: isize,
        max_array_len: isize,
        max_map_len: isize,
        max_ext_len: isize,
    ) -> PyResult<Py<PyAny>> {
        let kwargs = PyDict::new(py);
        kwargs.set_item("file_like", file_like.unwrap_or_else(|| py.None()))?;
        kwargs.set_item("read_size", read_size)?;
        kwargs.set_item("use_list", use_list)?;
        kwargs.set_item("raw", raw)?;
        kwargs.set_item("timestamp", timestamp)?;
        kwargs.set_item("strict_map_key", strict_map_key)?;
        kwargs.set_item("object_hook", object_hook.unwrap_or_else(|| py.None()))?;
        kwargs.set_item(
            "object_pairs_hook",
            object_pairs_hook.unwrap_or_else(|| py.None()),
        )?;
        kwargs.set_item("list_hook", list_hook.unwrap_or_else(|| py.None()))?;
        kwargs.set_item("unicode_errors", unicode_errors.unwrap_or("strict"))?;
        kwargs.set_item("max_buffer_size", max_buffer_size)?;
        let ext_hook = match ext_hook {
            Some(ext_hook) => ext_hook,
            None => py.import("msgpack.fallback")?.getattr("ExtType")?.unbind(),
        };
        kwargs.set_item("ext_hook", ext_hook)?;
        kwargs.set_item("max_str_len", max_str_len)?;
        kwargs.set_item("max_bin_len", max_bin_len)?;
        kwargs.set_item("max_array_len", max_array_len)?;
        kwargs.set_item("max_map_len", max_map_len)?;
        kwargs.set_item("max_ext_len", max_ext_len)?;

        let fallback_unpacker = py.import("msgpack.fallback")?.getattr("Unpacker")?;
        Ok(fallback_unpacker.call((), Some(&kwargs))?.unbind())
    }
}

#[pymethods]
impl Packer {
    #[new]
    #[pyo3(
        signature = (
            *,
            default=None,
            use_single_float=None,
            autoreset=None,
            use_bin_type=None,
            strict_types=None,
            r#datetime=None,
            unicode_errors=None,
            buf_size=None
        )
    )]
    fn new(
        py: Python<'_>,
        default: Option<PyObject>,
        use_single_float: Option<PyObject>,
        autoreset: Option<PyObject>,
        use_bin_type: Option<PyObject>,
        strict_types: Option<PyObject>,
        r#datetime: Option<PyObject>,
        unicode_errors: Option<&str>,
        buf_size: Option<usize>,
    ) -> PyResult<Self> {
        let use_single_float = if let Some(obj) = use_single_float {
            obj.bind(py).is_truthy()?
        } else {
            false
        };
        let autoreset = if let Some(obj) = autoreset {
            obj.bind(py).is_truthy()?
        } else {
            true
        };
        let use_bin_type = if let Some(obj) = use_bin_type {
            obj.bind(py).is_truthy()?
        } else {
            true
        };
        let strict_types = if let Some(obj) = strict_types {
            obj.bind(py).is_truthy()?
        } else {
            false
        };
        let r#datetime = if let Some(obj) = r#datetime {
            obj.bind(py).is_truthy()?
        } else {
            false
        };

        let kwargs = PyDict::new(py);
        kwargs.set_item("default", default.unwrap_or_else(|| py.None()))?;
        kwargs.set_item("use_single_float", use_single_float)?;
        kwargs.set_item("autoreset", autoreset)?;
        kwargs.set_item("use_bin_type", use_bin_type)?;
        kwargs.set_item("strict_types", strict_types)?;
        kwargs.set_item("datetime", r#datetime)?;
        kwargs.set_item("unicode_errors", unicode_errors.unwrap_or("strict"))?;
        kwargs.set_item("buf_size", buf_size)?;

        let fallback = py.import("msgpack.fallback")?;
        let fallback_packer = fallback.getattr("Packer")?;
        let inner = fallback_packer.call((), Some(&kwargs))?;
        Ok(Self {
            inner: inner.unbind(),
        })
    }

    #[pymethods]
    impl Unpacker {
        #[new]
        #[allow(clippy::too_many_arguments)]
        #[pyo3(
            signature = (
                file_like=None,
                *,
                read_size=0,
                use_list=true,
                raw=false,
                timestamp=0,
                strict_map_key=true,
                object_hook=None,
                object_pairs_hook=None,
                list_hook=None,
                unicode_errors=None,
                max_buffer_size=100 * 1024 * 1024,
                ext_hook=None,
                max_str_len=-1,
                max_bin_len=-1,
                max_array_len=-1,
                max_map_len=-1,
                max_ext_len=-1
            )
        )]
        fn new(
            py: Python<'_>,
            file_like: Option<PyObject>,
            read_size: usize,
            use_list: bool,
            raw: bool,
            timestamp: i64,
            strict_map_key: bool,
            object_hook: Option<PyObject>,
            object_pairs_hook: Option<PyObject>,
            list_hook: Option<PyObject>,
            unicode_errors: Option<&str>,
            max_buffer_size: usize,
            ext_hook: Option<PyObject>,
            max_str_len: isize,
            max_bin_len: isize,
            max_array_len: isize,
            max_map_len: isize,
            max_ext_len: isize,
        ) -> PyResult<Self> {
            Ok(Self {
                inner: Self::new_inner(
                    py,
                    file_like,
                    read_size,
                    use_list,
                    raw,
                    timestamp,
                    strict_map_key,
                    object_hook,
                    object_pairs_hook,
                    list_hook,
                    unicode_errors,
                    max_buffer_size,
                    ext_hook,
                    max_str_len,
                    max_bin_len,
                    max_array_len,
                    max_map_len,
                    max_ext_len,
                )?,
            })
        }

        #[allow(clippy::too_many_arguments)]
        #[pyo3(
            signature = (
                file_like=None,
                *,
                read_size=0,
                use_list=true,
                raw=false,
                timestamp=0,
                strict_map_key=true,
                object_hook=None,
                object_pairs_hook=None,
                list_hook=None,
                unicode_errors=None,
                max_buffer_size=100 * 1024 * 1024,
                ext_hook=None,
                max_str_len=-1,
                max_bin_len=-1,
                max_array_len=-1,
                max_map_len=-1,
                max_ext_len=-1
            )
        )]
        fn __init__(
            &mut self,
            py: Python<'_>,
            file_like: Option<PyObject>,
            read_size: usize,
            use_list: bool,
            raw: bool,
            timestamp: i64,
            strict_map_key: bool,
            object_hook: Option<PyObject>,
            object_pairs_hook: Option<PyObject>,
            list_hook: Option<PyObject>,
            unicode_errors: Option<&str>,
            max_buffer_size: usize,
            ext_hook: Option<PyObject>,
            max_str_len: isize,
            max_bin_len: isize,
            max_array_len: isize,
            max_map_len: isize,
            max_ext_len: isize,
        ) -> PyResult<()> {
            self.inner = Self::new_inner(
                py,
                file_like,
                read_size,
                use_list,
                raw,
                timestamp,
                strict_map_key,
                object_hook,
                object_pairs_hook,
                list_hook,
                unicode_errors,
                max_buffer_size,
                ext_hook,
                max_str_len,
                max_bin_len,
                max_array_len,
                max_map_len,
                max_ext_len,
            )?;
            Ok(())
        }

        fn feed(&self, py: Python<'_>, next_bytes: &Bound<'_, PyAny>) -> PyResult<()> {
            self.inner.bind(py).call_method1("feed", (next_bytes,))?;
            Ok(())
        }

        fn read_bytes(&self, py: Python<'_>, n: usize) -> PyResult<PyObject> {
            Ok(self.inner.bind(py).call_method1("read_bytes", (n,))?.into())
        }

        fn skip(&self, py: Python<'_>) -> PyResult<()> {
            self.inner.bind(py).call_method0("skip")?;
            Ok(())
        }

        fn unpack(&self, py: Python<'_>) -> PyResult<PyObject> {
            Ok(self.inner.bind(py).call_method0("unpack")?.into())
        }

        fn read_array_header(&self, py: Python<'_>) -> PyResult<PyObject> {
            Ok(self.inner.bind(py).call_method0("read_array_header")?.into())
        }

        fn read_map_header(&self, py: Python<'_>) -> PyResult<PyObject> {
            Ok(self.inner.bind(py).call_method0("read_map_header")?.into())
        }

        fn tell(&self, py: Python<'_>) -> PyResult<PyObject> {
            Ok(self.inner.bind(py).call_method0("tell")?.into())
        }

        fn __iter__(slf: PyRef<'_, Self>) -> Py<Self> {
            slf.into()
        }

        fn __next__(&self, py: Python<'_>) -> PyResult<Option<PyObject>> {
            match self.inner.bind(py).call_method0("__next__") {
                Ok(value) => Ok(Some(value.into())),
                Err(err) => {
                    if err.is_instance_of::<PyStopIteration>(py) {
                        Ok(None)
                    } else {
                        Err(err)
                    }
                }
            }
        }

        #[pyo3(name = "next")]
        fn next_py(&self, py: Python<'_>) -> PyResult<PyObject> {
            Ok(self.inner.bind(py).call_method0("__next__")?.into())
        }

        fn __getattr__(&self, py: Python<'_>, name: &str) -> PyResult<PyObject> {
            Ok(self.inner.bind(py).getattr(name)?.into())
        }
    }

    fn pack(&self, py: Python<'_>, obj: &Bound<'_, PyAny>) -> PyResult<PyObject> {
        let inner = self.inner.bind(py);
        let previous_bytes = inner.call_method0("bytes")?;
        match inner.call_method1("pack", (obj,)) {
            Ok(value) => Ok(value.into()),
            Err(err) => {
                if err.is_instance_of::<PyBufferError>(py)
                    && !inner.getattr("_autoreset")?.is_truthy()?
                {
                    let bytes_io = py.import("io")?.getattr("BytesIO")?;
                    let restored = bytes_io.call1((previous_bytes,))?;
                    restored.call_method1("seek", (0, 2))?;
                    inner.setattr("_buffer", restored)?;
                }
                Err(err)
            }
        }
    }

    fn pack_map_pairs(&self, py: Python<'_>, pairs: &Bound<'_, PyAny>) -> PyResult<PyObject> {
        Ok(self
            .inner
            .bind(py)
            .call_method1("pack_map_pairs", (pairs,))?
            .into())
    }

    fn pack_array_header(&self, py: Python<'_>, n: usize) -> PyResult<PyObject> {
        Ok(self
            .inner
            .bind(py)
            .call_method1("pack_array_header", (n,))?
            .into())
    }

    fn pack_map_header(&self, py: Python<'_>, n: usize) -> PyResult<PyObject> {
        Ok(self
            .inner
            .bind(py)
            .call_method1("pack_map_header", (n,))?
            .into())
    }

    fn pack_ext_type(
        &self,
        py: Python<'_>,
        typecode: i64,
        data: &Bound<'_, PyAny>,
    ) -> PyResult<PyObject> {
        Ok(self
            .inner
            .bind(py)
            .call_method1("pack_ext_type", (typecode, data))?
            .into())
    }

    fn bytes(&self, py: Python<'_>) -> PyResult<PyObject> {
        Ok(self.inner.bind(py).call_method0("bytes")?.into())
    }

    fn reset(&self, py: Python<'_>) -> PyResult<()> {
        self.inner.bind(py).call_method0("reset")?;
        Ok(())
    }

    fn getbuffer(&self, py: Python<'_>) -> PyResult<PyObject> {
        Ok(self.inner.bind(py).call_method0("getbuffer")?.into())
    }

    fn __bytes__(&self, py: Python<'_>) -> PyResult<PyObject> {
        Ok(self.inner.bind(py).call_method0("bytes")?.into())
    }

    fn __getattr__(&self, py: Python<'_>, name: &str) -> PyResult<PyObject> {
        Ok(self.inner.bind(py).getattr(name)?.into())
    }
}

#[pyfunction]
fn default_read_extended_type(typecode: i8, _data: &Bound<'_, PyAny>) -> PyResult<PyObject> {
    Err(PyNotImplementedError::new_err(format!(
        "Cannot decode extended type with typecode={typecode}"
    )))
}

#[pymodule]
fn _cmsgpack(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    let fallback = py.import("msgpack.fallback")?;
    let exceptions = py.import("msgpack.exceptions")?;
    let datetime = py.import("datetime")?;

    m.add_class::<Packer>()?;
    m.add_class::<Unpacker>()?;
    for name in ["unpackb"] {
        m.add(name, fallback.getattr(name)?)?;
    }
    for name in ["BufferFull", "ExtraData", "FormatError", "OutOfData", "StackError"] {
        m.add(name, exceptions.getattr(name)?)?;
    }
    m.add("datetime", datetime)?;
    m.add_function(wrap_pyfunction!(default_read_extended_type, m)?)?;
    Ok(())
}
