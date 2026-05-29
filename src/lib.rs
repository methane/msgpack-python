use pyo3::exceptions::PyNotImplementedError;
use pyo3::prelude::*;
use pyo3::types::PyDict;

#[pyclass(module = "msgpack._cmsgpack")]
struct Packer {
    inner: Py<PyAny>,
}

#[pymethods]
impl Packer {
    #[new]
    #[pyo3(
        signature = (
            *,
            default=None,
            use_single_float=false,
            autoreset=true,
            use_bin_type=true,
            strict_types=false,
            datetime=false,
            unicode_errors=None,
            buf_size=None
        )
    )]
    fn new(
        py: Python<'_>,
        default: Option<PyObject>,
        use_single_float: bool,
        autoreset: bool,
        use_bin_type: bool,
        strict_types: bool,
        r#datetime: bool,
        unicode_errors: Option<&str>,
        buf_size: Option<usize>,
    ) -> PyResult<Self> {
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

    fn pack(&self, py: Python<'_>, obj: &Bound<'_, PyAny>) -> PyResult<PyObject> {
        Ok(self.inner.bind(py).call_method1("pack", (obj,))?.into())
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
    for name in ["Unpacker", "unpackb"] {
        m.add(name, fallback.getattr(name)?)?;
    }
    for name in ["BufferFull", "ExtraData", "FormatError", "OutOfData", "StackError"] {
        m.add(name, exceptions.getattr(name)?)?;
    }
    m.add("datetime", datetime)?;
    m.add_function(wrap_pyfunction!(default_read_extended_type, m)?)?;
    Ok(())
}
