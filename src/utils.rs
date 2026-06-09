use pyo3::prelude::*;

pub fn register_utils_module(py: Python) -> PyResult<Py<PyModule>> {
    let utils = PyModule::new(py, "utils")?;

    // Token types
    utils.add("TOKEN_ACCESS", "access")?;
    utils.add("TOKEN_REFRESH", "refresh")?;

    //secret size
    utils.add("DEFAULT_SECRET_SIZE", 32)?;
    utils.add("MIN_SECRET_SIZE", 16)?;

    Ok(utils.into())
}
