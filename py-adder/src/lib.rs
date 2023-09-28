use pyo3::{pyfunction, wrap_pyfunction, Python, types::PyModule, PyResult, pymodule};
use backend::add;

#[pyfunction]
pub fn py_add(left: usize, right: usize) -> usize {
    add(left, right)
}

#[pymodule]
fn adder(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_add, m)?)?;

    Ok(())
}