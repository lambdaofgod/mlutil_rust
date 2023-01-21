mod tokenization;

use pyo3::prelude::*;
use pyo3::types::*;

#[pyfunction]
fn tokenize_python_code(python_code: &PyString) -> PyResult<Vec<String>> {
    let python_code_string: String = python_code.to_string(); // make sure the bytes are UTF-8
    Ok(crate::tokenization::tokenize(python_code_string))
}

#[pymodule]
fn mlutil_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(tokenize_python_code, m)?)?;
    Ok(())
}
