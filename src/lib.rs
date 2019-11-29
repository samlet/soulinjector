pub mod hex_util;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
/// Formats the sum of two numbers as string
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}
#[pyfunction]
fn sum_as_string2(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// This module is a python module implemented in Rust.
#[pymodule]
fn soulinjector(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(sum_as_string))?;
    m.add_wrapped(wrap_pyfunction!(sum_as_string2))?;

    Ok(())
}

