use pyo3::prelude::*;

pub fn register_algorithms_module(py: Python) -> PyResult<Py<PyModule>>{
    let alg_module = PyModule::new(py, "algorithms")?;

    // HMAC
    alg_module.add("HS256", "HS256")?;
    alg_module.add("HS384", "HS384")?;
    alg_module.add("HS512", "HS512")?;

    // RSA
    alg_module.add("RS256", "RS256")?;
    alg_module.add("RS384", "RS384")?;
    alg_module.add("RS512", "RS512")?;

    // ECDSA
    alg_module.add("ES256", "ES256")?;
    alg_module.add("ES384", "ES384")?;

    // RSA-PSS
    alg_module.add("PS256", "PS256")?;
    alg_module.add("PS384", "PS384")?;
    alg_module.add("PS512", "PS512")?;

    // Edwards Curve
    alg_module.add("EDDSA", "EdDSA")?;

    Ok(alg_module.into())
}
