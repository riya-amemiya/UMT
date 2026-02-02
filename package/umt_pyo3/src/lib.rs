use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
mod umt_pyo3 {
    use pyo3::prelude::*;
    use umt_rust::array::umt_range;

    /// Formats the sum of two numbers as string.
    #[pyfunction]
    fn range(min: i32, max: i32) -> PyResult<Vec<i32>> {
        Ok(umt_range(min, max))
    }
}
