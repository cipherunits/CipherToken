use pyo3::prelude::*;
use rand::rngs::OsRng;
use rand::RngCore;
use base64::{engine::general_purpose, Engine as _};
use tokio::task;
use pyo3_asyncio::tokio::future_into_py;
use rsa::{RsaPrivateKey, RsaPublicKey, pkcs8::{EncodePrivateKey, EncodePublicKey, LineEnding}};



#[pyfunction]
/// generate random secret key
pub fn secret_key() -> PyResult<String> {
    generate_hmac_secret(32)
}

#[pyfunction]
/// generate secret key with custom size
pub fn secret_key_with_size(size: usize) -> PyResult<String> {
    generate_hmac_secret(size)
}

#[pyfunction]
/// generate random secret key for HMAC algorithms
pub fn generate_hmac_secret(size: usize) -> PyResult<String> {
    if size < 16 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Secret key size must be at least 16 bytes",
        ));
    }

    let mut key = vec![0u8; size];
    OsRng.fill_bytes(&mut key);
    Ok(general_purpose::STANDARD.encode(key))
}

// Async version of helper functions
#[pyfunction]
pub fn generate_hmac_secret_async<'a>(py: Python<'a>, size: usize) -> PyResult<&'a PyAny> {
    future_into_py(py, async move {
        if size < 16 {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "Secret key size must be at least 16 bytes",
            ));
        }

        let mut key = vec![0u8; size];
        task::spawn_blocking(move || {
            OsRng.fill_bytes(&mut key);
            general_purpose::STANDARD.encode(key)
        })
        .await
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
    })
}



#[pyfunction]
pub fn generate_rsa_keypair(bits: Option<usize>) -> PyResult<(String, String)> {
    let bits = bits.unwrap_or(2048);

    if bits < 2048 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "RSA key size must be at least 2048 bits",
        ));
    }

    let mut rng = OsRng;

    let private_key = RsaPrivateKey::new(&mut rng, bits)
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

    let public_key = RsaPublicKey::from(&private_key);

    let private_pem = private_key
        .to_pkcs8_pem(LineEnding::LF)
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?
        .to_string();

    let public_pem = public_key
        .to_public_key_pem(LineEnding::LF)
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

    Ok((private_pem, public_pem))
}


pub fn register_secret_module(py: Python) -> PyResult<Py<PyModule>> {
    let m = PyModule::new(py, "secret")?;
    m.add_function(wrap_pyfunction!(secret_key, m)?)?;
    m.add_function(wrap_pyfunction!(secret_key_with_size, m)?)?;
    m.add_function(wrap_pyfunction!(generate_hmac_secret, m)?)?;
    m.add_function(wrap_pyfunction!(generate_hmac_secret_async, m)?)?;
    m.add_function(wrap_pyfunction!(generate_rsa_keypair, m)?)?;
    Ok(m.into())
}
