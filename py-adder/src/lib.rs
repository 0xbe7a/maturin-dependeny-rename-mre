use pyo3::{pyfunction, wrap_pyfunction, Python, types::PyModule, PyResult, pymodule};
use some_other_crate::wrapped_add;

#[pyfunction]
pub fn py_add(left: usize, right: usize) -> usize {
    wrapped_add(left, right)
}

#[pymodule]
fn adder(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_add, m)?)?;

    Ok(())
}