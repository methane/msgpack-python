use pyo3::exceptions::PyNotImplementedError;
use pyo3::prelude::*;

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

    for name in ["Packer", "Unpacker", "unpackb"] {
        m.add(name, fallback.getattr(name)?)?;
    }
    for name in ["BufferFull", "ExtraData", "FormatError", "OutOfData", "StackError"] {
        m.add(name, exceptions.getattr(name)?)?;
    }
    m.add("datetime", datetime)?;
    m.add_function(wrap_pyfunction!(default_read_extended_type, m)?)?;
    Ok(())
}
