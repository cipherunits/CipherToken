use pyo3::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

#[pyfunction]
pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[pyfunction]
pub fn seconds(n: u64) -> u64 { n }

#[pyfunction]
pub fn minutes(n: u64) -> u64 { n.saturating_mul(60) }

#[pyfunction]
pub fn hours(n: u64) -> u64 { n.saturating_mul(60).saturating_mul(60) }

#[pyfunction]
pub fn days(n: u64) -> u64 {
    n.saturating_mul(60)
        .saturating_mul(60)
        .saturating_mul(24)
}

#[pyfunction]
pub fn weeks(n: u64) -> u64 {
    n.saturating_mul(60)
        .saturating_mul(60)
        .saturating_mul(24)
        .saturating_mul(7)
}


pub fn register_time_module(py: Python) -> PyResult<Py<PyModule>> {
    let time = PyModule::new(py, "time")?;
    time.add_function(wrap_pyfunction!(now, time)?)?;
    time.add_function(wrap_pyfunction!(seconds, time)?)?;
    time.add_function(wrap_pyfunction!(minutes, time)?)?;
    time.add_function(wrap_pyfunction!(hours, time)?)?;
    time.add_function(wrap_pyfunction!(days, time)?)?;
    time.add_function(wrap_pyfunction!(weeks, time)?)?;
    Ok(time.into())
}
