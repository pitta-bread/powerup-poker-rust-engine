use pyo3::prelude::*;

#[pyfunction]
fn ping() -> &'static str {
    "pong from rust"
}

#[pymodule]
fn powerup_poker_rust_engine(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(ping, m)?)?;
    Ok(())
}
