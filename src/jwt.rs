use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3_asyncio::tokio::future_into_py;

use crate::CipherToken;

#[pyfunction]
#[pyo3(signature = (token, payload=None))]
pub fn access(py: Python, token: PyRef<CipherToken>, payload: Option<&PyDict>) -> PyResult<String> {
    token.access(py, payload)
}

#[pyfunction]
#[pyo3(signature = (token, payload=None))]
pub fn refresh(py: Python, token: PyRef<CipherToken>, payload: Option<&PyDict>) -> PyResult<String> {
    token.refresh(py, payload)
}

#[pyfunction]
#[pyo3(signature = (token, refresh_token, payload=None))]
pub fn rotation(
    py: Python,
    token: PyRef<CipherToken>,
    refresh_token: String,
    payload: Option<&PyDict>,
) -> PyResult<(String, String)> {
    token.rotation(py, refresh_token, payload)
}

#[pyfunction]
#[pyo3(signature = (token, payload=None))]
pub fn access_async<'a>(
    py: Python<'a>,
    token: PyRef<'a, CipherToken>,
    payload: Option<&PyDict>,
) -> PyResult<&'a PyAny> {
    let token_instance = token.clone_token();
    let payload_cloned = payload.map(|dict| dict.into());
    future_into_py(py, async move { token_instance.access_async_inner(payload_cloned).await })
}

#[pyfunction]
#[pyo3(signature = (token, payload=None))]
pub fn refresh_async<'a>(
    py: Python<'a>,
    token: PyRef<'a, CipherToken>,
    payload: Option<&PyDict>,
) -> PyResult<&'a PyAny> {
    let token_instance = token.clone_token();
    let payload_cloned = payload.map(|dict| dict.into());
    future_into_py(py, async move { token_instance.refresh_async_inner(payload_cloned).await })
}

#[pyfunction]
#[pyo3(signature = (token, refresh_token, payload=None))]
pub fn rotation_async<'a>(
    py: Python<'a>,
    token: PyRef<'a, CipherToken>,
    refresh_token: String,
    payload: Option<&PyDict>,
) -> PyResult<&'a PyAny> {
    let token_instance = token.clone_token();
    let payload_cloned = payload.map(|dict| dict.into());
    future_into_py(py, async move {
        let claims_dict = token_instance.decode_async_inner(&refresh_token).await?;

        let token_type: String = Python::with_gil(|py| {
            claims_dict
                .as_ref(py)
                .get_item("token")?
                .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("Token type not found"))?
                .extract()
        })?;

        if token_type != "refresh" {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "Only refresh tokens can be used for rotation",
            ));
        }

        let new_access = token_instance.access_async_inner(payload_cloned.clone()).await?;
        let new_refresh = token_instance.refresh_async_inner(payload_cloned).await?;
        Ok((new_access, new_refresh))
    })
}

pub fn register_jwt_module(py: Python) -> PyResult<Py<PyModule>> {
    let jwt = PyModule::new(py, "jwt")?;
    jwt.add("TOKEN_ACCESS", "access")?;
    jwt.add("TOKEN_REFRESH", "refresh")?;
    jwt.add_function(wrap_pyfunction!(access, jwt)?)?;
    jwt.add_function(wrap_pyfunction!(refresh, jwt)?)?;
    jwt.add_function(wrap_pyfunction!(rotation, jwt)?)?;
    jwt.add_function(wrap_pyfunction!(access_async, jwt)?)?;
    jwt.add_function(wrap_pyfunction!(refresh_async, jwt)?)?;
    jwt.add_function(wrap_pyfunction!(rotation_async, jwt)?)?;
    Ok(jwt.into())
}
