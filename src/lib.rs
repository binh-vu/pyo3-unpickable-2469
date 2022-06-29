use pyo3::prelude::*;

/// square of a number
#[pyfunction]
fn square(x: f64) -> f64 {
    x * x
}

/// cube of a number
#[pyfunction]
fn cube(x: f64) -> f64 {
    x * x * x
}

pub fn register_submodule(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    let submodule = PyModule::new(py, "sub1")?;

    submodule.add_function(wrap_pyfunction!(cube, submodule)?)?;
    m.add_submodule(submodule)?;

    Ok(())
}

#[pymodule]
fn demo(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(square, m)?)?;
    register_submodule(py, m)?;
    Ok(())
}
