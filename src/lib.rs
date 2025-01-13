use pyo3::prelude::*;

#[pymodule]
fn repro(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}
