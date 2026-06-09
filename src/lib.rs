use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use pyo3_asyncio::tokio::future_into_py;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use tokio::task;
use tokio::runtime::Builder;


mod secret;
mod time;
mod algorithms;
mod utils;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: Option<i128>,
    pub exp: usize,
    pub ttl: usize,
    pub token: String,
    pub jti: String,
    // extra payload
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[pyclass]
pub struct CipherToken {
    secret: String,
    algorithm: Algorithm,
    access_ttl: u64,
    refresh_ttl: u64,
}

// supported algorithm
fn parse_algorithm(alg_str: &str) -> PyResult<Algorithm> {
    match alg_str.to_uppercase().as_str() {
        "HS256" => Ok(Algorithm::HS256),
        "HS384" => Ok(Algorithm::HS384),
        "HS512" => Ok(Algorithm::HS512),
        "RS256" => Ok(Algorithm::RS256),
        "RS384" => Ok(Algorithm::RS384),
        "RS512" => Ok(Algorithm::RS512),
        "ES256" => Ok(Algorithm::ES256),
        "ES384" => Ok(Algorithm::ES384),
        "PS256" => Ok(Algorithm::PS256),
        "PS384" => Ok(Algorithm::PS384),
        "PS512" => Ok(Algorithm::PS512),
        "EDDSA" => Ok(Algorithm::EdDSA),
        _ => Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Unsupported algorithm: {}",
            alg_str
        ))),
    }
}

// Sync Implementation
#[pymethods]
impl CipherToken {
    
    #[new]
    pub fn new(
        secret: String,
        algorithm: String,
        access_ttl: u64,
        refresh_ttl: u64,
    ) -> PyResult<Self> {
        let alg = parse_algorithm(&algorithm)?;

        Ok(CipherToken {
            secret,
            algorithm: alg,
            access_ttl,
            refresh_ttl,
        })
    }

    #[pyo3(signature = (ttl_time, token_type, user_id=None, extra_payload=None))]
    pub fn create_token(
        &self,
        py: Python,
        ttl_time: u64,
        token_type: String,
        user_id: Option<i128>,
        extra_payload: Option<&PyDict>,
    ) -> PyResult<String> {
        let uuid = Uuid::new_v4();

        let exp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?
            .as_secs()
            + ttl_time;

        
        let mut extra_map = Map::new();

        if let Some(py_dict) = extra_payload {
            for (key, value) in py_dict.iter() {
                let key_str = key.extract::<String>()?;

                
                let json_value = python_to_json(py, value)?;
                extra_map.insert(key_str, json_value);
            }
        }

        let claims = Claims {
            id: user_id,
            exp: exp as usize,
            ttl: ttl_time as usize,
            token: token_type,
            jti: uuid.to_string(),
            extra: extra_map,
        };

        let encoding_key = match self.algorithm {
            Algorithm::HS256 | Algorithm::HS384 | Algorithm::HS512 => {
                EncodingKey::from_secret(self.secret.as_bytes())
            }
            Algorithm::RS256
            | Algorithm::RS384
            | Algorithm::RS512
            | Algorithm::PS256
            | Algorithm::PS384
            | Algorithm::PS512 => EncodingKey::from_rsa_pem(self.secret.as_bytes())
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?,
            Algorithm::ES256 | Algorithm::ES384 => EncodingKey::from_ec_pem(self.secret.as_bytes())
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?,
            Algorithm::EdDSA => EncodingKey::from_ed_pem(self.secret.as_bytes())
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?,
        };

        let token = encode(
            &Header::new(self.algorithm),
            &claims,
            &encoding_key,
        )
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

        Ok(token)
    }

    /// create access token - sync
    #[pyo3(signature = (user_id, extra_payload=None))]
    pub fn access(
        &self,
        py: Python,
        user_id: i128,
        extra_payload: Option<&PyDict>,
    ) -> PyResult<String> {
        self.create_token(
            py,
            self.access_ttl,
            "access".to_string(),
            Some(user_id),
            extra_payload,
        )
    }

    /// create refresh token - sync
    #[pyo3(signature = (user_id, extra_payload=None))]
    pub fn refresh(
        &self,
        py: Python,
        user_id: i128,
        extra_payload: Option<&PyDict>,
    ) -> PyResult<String> {
        self.create_token(
            py,
            self.refresh_ttl,
            "refresh".to_string(),
            Some(user_id),
            extra_payload,
        )
    }

    /// decode token - sync
    pub fn decode<'a>(&self, py: Python<'a>, token: &str) -> PyResult<Py<PyDict>> {
        let mut validation = Validation::new(self.algorithm);
        validation.validate_exp = true;
        validation.required_spec_claims.clear();

        let decoding_key = match self.algorithm {
            Algorithm::HS256 | Algorithm::HS384 | Algorithm::HS512 => {
                DecodingKey::from_secret(self.secret.as_bytes())
            }
            Algorithm::RS256
            | Algorithm::RS384
            | Algorithm::RS512
            | Algorithm::PS256
            | Algorithm::PS384
            | Algorithm::PS512 => DecodingKey::from_rsa_pem(self.secret.as_bytes())
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?,
            Algorithm::ES256 | Algorithm::ES384 => DecodingKey::from_ec_pem(self.secret.as_bytes())
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?,
            Algorithm::EdDSA => DecodingKey::from_ed_pem(self.secret.as_bytes())
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?,
        };

        let token_data: TokenData<Claims> =
            decode::<Claims>(token, &decoding_key, &validation)
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

        let claims = token_data.claims;
        let dict = PyDict::new(py);

        if let Some(id) = claims.id {
            dict.set_item("id", id)?;
        }
        dict.set_item("exp", claims.exp)?;
        dict.set_item("ttl", claims.ttl)?;
        dict.set_item("token", claims.token)?;
        dict.set_item("jti", claims.jti)?;

        for (key, value) in claims.extra {
            let py_value = json_to_python(py, &value)?;
            dict.set_item(key, py_value)?;
        }

        Ok(dict.into())
    }

    /// token rotation - sync
    #[pyo3(signature = (refresh_token, extra_payload=None))]
    pub fn rotation(
        &self,
        py: Python,
        refresh_token: String,
        extra_payload: Option<&PyDict>,
    ) -> PyResult<(String, String)> {
        let claims_dict = self.decode(py, &refresh_token)?;
        let claims_dict = claims_dict.as_ref(py);
        
        let token_type: String = claims_dict
            .get_item("token")?
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("Token type not found"))?
            .extract()?;

        if token_type != "refresh" {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "Only refresh tokens can be used for rotation",
            ));
        }

        let user_id: i128 = claims_dict
            .get_item("id")?
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("User ID not found"))?
            .extract()?;

        let new_access = self.access(py, user_id, extra_payload.clone())?;
        let new_refresh = self.refresh(py, user_id, extra_payload)?;

        Ok((new_access, new_refresh))
    }

    /// verify token - sync
    pub fn verify(&self, token: &str) -> PyResult<bool> {
        let mut validation = Validation::new(self.algorithm);
        validation.validate_exp = true;

        let decoding_key = match self.algorithm {
            Algorithm::HS256 | Algorithm::HS384 | Algorithm::HS512 => {
                DecodingKey::from_secret(self.secret.as_bytes())
            }
            Algorithm::RS256
            | Algorithm::RS384
            | Algorithm::RS512
            | Algorithm::PS256
            | Algorithm::PS384
            | Algorithm::PS512 => DecodingKey::from_rsa_pem(self.secret.as_bytes())
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?,
            Algorithm::ES256 | Algorithm::ES384 => DecodingKey::from_ec_pem(self.secret.as_bytes())
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?,
            Algorithm::EdDSA => DecodingKey::from_ed_pem(self.secret.as_bytes())
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?,
        };

        match decode::<Claims>(token, &decoding_key, &validation) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// inspect token - sync
    pub fn inspect<'a>(&self, py: Python<'a>, token: &str) -> PyResult<Py<PyDict>> {
        let mut validation = Validation::default();
        validation.insecure_disable_signature_validation();
        validation.validate_exp = false;

        let decoding_key = match self.algorithm {
            Algorithm::HS256 | Algorithm::HS384 | Algorithm::HS512 => {
                DecodingKey::from_secret(self.secret.as_bytes())
            }
            Algorithm::RS256
            | Algorithm::RS384
            | Algorithm::RS512
            | Algorithm::PS256
            | Algorithm::PS384
            | Algorithm::PS512 => DecodingKey::from_rsa_pem(self.secret.as_bytes())
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?,
            Algorithm::ES256 | Algorithm::ES384 => DecodingKey::from_ec_pem(self.secret.as_bytes())
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?,
            Algorithm::EdDSA => DecodingKey::from_ed_pem(self.secret.as_bytes())
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?,
        };

        match decode::<Claims>(token, &decoding_key, &validation) {
            Ok(decoded) => {
                let dict = PyDict::new(py);
                dict.set_item("id", decoded.claims.id)?;
                dict.set_item("exp", decoded.claims.exp)?;
                dict.set_item("token", decoded.claims.token)?;
                dict.set_item("jti", decoded.claims.jti)?;
                dict.set_item("ttl", decoded.claims.ttl)?;

                for (key, value) in decoded.claims.extra {
                    let py_value = json_to_python(py, &value)?;
                    dict.set_item(key, py_value)?;
                }

                Ok(dict.into())
            }
            Err(e) => Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Failed to inspect token: {}",
                e
            ))),
        }
    }

    /// remaining time - sync
    pub fn remaining_time(&self, py: Python, token: &str) -> PyResult<Option<i64>> {
        let dict = self.inspect(py, token)?;
        let dict = dict.as_ref(py);

        if let Some(exp) = dict.get_item("exp")? {
            let exp_secs: i64 = exp.extract()?;
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?
                .as_secs() as i64;

            let remaining = exp_secs - now;
            if remaining > 0 {
                Ok(Some(remaining))
            } else {
                Ok(Some(0))
            }
        } else {
            Ok(None)
        }
    }

    /// algorithm name - sync
    pub fn algorithm_name(&self) -> String {
        match self.algorithm {
            Algorithm::HS256 => "HS256",
            Algorithm::HS384 => "HS384",
            Algorithm::HS512 => "HS512",
            Algorithm::RS256 => "RS256",
            Algorithm::RS384 => "RS384",
            Algorithm::RS512 => "RS512",
            Algorithm::ES256 => "ES256",
            Algorithm::ES384 => "ES384",
            Algorithm::PS256 => "PS256",
            Algorithm::PS384 => "PS384",
            Algorithm::PS512 => "PS512",
            Algorithm::EdDSA => "EDDSA",
        }
        .to_string()
    }

    /// getter for secret (masked)
    #[getter]
    pub fn secret(&self) -> String {
        if self.secret.len() > 8 {
            format!("{}...", &self.secret[0..8])
        } else {
            "***".to_string()
        }
    }

    #[getter]
    pub fn access_ttl(&self) -> u64 {
        self.access_ttl
    }

    #[getter]
    pub fn refresh_ttl(&self) -> u64 {
        self.refresh_ttl
    }

    // Async methods
    /// create access token - async
    #[pyo3(signature = (user_id, extra_payload=None))]
    pub fn access_async<'a>(
        &'a self,
        py: Python<'a>,
        user_id: i128,
        extra_payload: Option<&PyDict>,
    ) -> PyResult<&'a PyAny> {
        let secret = self.secret.clone();
        let algorithm = self.algorithm;
        let access_ttl = self.access_ttl;
        let extra_payload_cloned = extra_payload.map(|dict| dict.into());

        future_into_py(py, async move {
            let uuid = Uuid::new_v4();
            let exp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?
                .as_secs()
                + access_ttl;

            let claims = Claims {
                id: Some(user_id),
                exp: exp as usize,
                ttl: access_ttl as usize,
                token: "access".to_string(),
                jti: uuid.to_string(),
                extra: extra_map_from_py(extra_payload_cloned).await?,
            };

            let encoding_key = create_encoding_key(&secret, algorithm)?;
            
            let token = task::spawn_blocking(move || {
                encode(&Header::new(algorithm), &claims, &encoding_key)
            })
            .await
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

            Ok(token)
        })
    }

    /// create refresh token - async
    #[pyo3(signature = (user_id, extra_payload=None))]
    pub fn refresh_async<'a>(
        &'a self,
        py: Python<'a>,
        user_id: i128,
        extra_payload: Option<&PyDict>,
    ) -> PyResult<&'a PyAny> {
        let secret = self.secret.clone();
        let algorithm = self.algorithm;
        let refresh_ttl = self.refresh_ttl;
        let extra_payload_cloned = extra_payload.map(|dict| dict.into());

        future_into_py(py, async move {
            let uuid = Uuid::new_v4();
            let exp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?
                .as_secs()
                + refresh_ttl;

            let claims = Claims {
                id: Some(user_id),
                exp: exp as usize,
                ttl: refresh_ttl as usize,
                token: "refresh".to_string(),
                jti: uuid.to_string(),
                extra: extra_map_from_py(extra_payload_cloned).await?,
            };

            let encoding_key = create_encoding_key(&secret, algorithm)?;
            
            let token = task::spawn_blocking(move || {
                encode(&Header::new(algorithm), &claims, &encoding_key)
            })
            .await
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

            Ok(token)
        })
    }

    /// decode token - async
    pub fn decode_async<'a>(
        &'a self,
        py: Python<'a>,
        token: String,
    ) -> PyResult<&'a PyAny> {
        let secret = self.secret.clone();
        let algorithm = self.algorithm;

        future_into_py(py, async move {
            let mut validation = Validation::new(algorithm);
            validation.validate_exp = true;
            validation.required_spec_claims.clear();

            let decoding_key = create_decoding_key(&secret, algorithm)?;
            
            let token_data: TokenData<Claims> = task::spawn_blocking(move || {
                decode::<Claims>(&token, &decoding_key, &validation)
            })
            .await
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

            let py_dict = Python::with_gil(|py| {
                let dict = PyDict::new(py);
                if let Some(id) = token_data.claims.id {
                    dict.set_item("id", id)?;
                }
                dict.set_item("exp", token_data.claims.exp)?;
                dict.set_item("ttl", token_data.claims.ttl)?;
                dict.set_item("token", token_data.claims.token)?;
                dict.set_item("jti", token_data.claims.jti)?;

                for (key, value) in token_data.claims.extra {
                    let py_value = json_to_python(py, &value)?;
                    dict.set_item(key, py_value)?;
                }
                Ok::<Py<PyDict>, PyErr>(dict.into())
            })?;

            Ok(py_dict)
        })
    }

    /// verify token - async
    pub fn verify_async<'a>(
        &'a self,
        py: Python<'a>,
        token: String,
    ) -> PyResult<&'a PyAny> {
        let secret = self.secret.clone();
        let algorithm = self.algorithm;

        future_into_py(py, async move {
            let mut validation = Validation::new(algorithm);
            validation.validate_exp = true;

            let decoding_key = create_decoding_key(&secret, algorithm)?;
            
            let result = task::spawn_blocking(move || {
                decode::<Claims>(&token, &decoding_key, &validation)
            })
            .await
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

            match result {
                Ok(_) => Ok(true),
                Err(_) => Ok(false),
            }
        })
    }

    /// token rotation - async
    #[pyo3(signature = (refresh_token, extra_payload=None))]
    pub fn rotation_async<'a>(
        &'a self,
        py: Python<'a>,
        refresh_token: String,
        extra_payload: Option<&PyDict>,
    ) -> PyResult<&'a PyAny> {
        let token_instance = self.clone_token();
        let extra_payload_cloned = extra_payload.map(|dict| dict.into());

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

            let user_id: i128 = Python::with_gil(|py| {
                claims_dict
                    .as_ref(py)
                    .get_item("id")?
                    .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("User ID not found"))?
                    .extract()
            })?;

            let new_access = token_instance.access_async_inner(user_id, extra_payload_cloned.clone()).await?;
            let new_refresh = token_instance.refresh_async_inner(user_id, extra_payload_cloned).await?;

            Ok((new_access, new_refresh))
        })
    }
}

// Helper functions for async operations
impl CipherToken {
    fn clone_token(&self) -> CipherToken {
        CipherToken {
            secret: self.secret.clone(),
            algorithm: self.algorithm,
            access_ttl: self.access_ttl,
            refresh_ttl: self.refresh_ttl,
        }
    }

    async fn access_async_inner(
        &self,
        user_id: i128,
        extra_payload: Option<Py<PyDict>>,
    ) -> PyResult<String> {
        let uuid = Uuid::new_v4();
        let exp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?
            .as_secs()
            + self.access_ttl;

        let extra_map = if let Some(py_dict) = extra_payload {
            Python::with_gil(|py| {
                let dict = py_dict.as_ref(py);
                let mut extra_map = Map::new();
                for (key, value) in dict.iter() {
                    let key_str = key.extract::<String>()?;
                    let json_value = python_to_json(py, value)?;
                    extra_map.insert(key_str, json_value);
                }
                Ok::<Map<String, Value>, PyErr>(extra_map)
            })?
        } else {
            Map::new()
        };

        let claims = Claims {
            id: Some(user_id),
            exp: exp as usize,
            ttl: self.access_ttl as usize,
            token: "access".to_string(),
            jti: uuid.to_string(),
            extra: extra_map,
        };
        let algorithm = self.algorithm;

        let encoding_key = create_encoding_key(&self.secret, self.algorithm)?;
        
        let token = task::spawn_blocking(move || {
            encode(&Header::new(algorithm), &claims, &encoding_key)
        })
        .await
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

        Ok(token)
    }

    async fn refresh_async_inner(
        &self,
        user_id: i128,
        extra_payload: Option<Py<PyDict>>,
    ) -> PyResult<String> {
        let uuid = Uuid::new_v4();
        let exp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?
            .as_secs()
            + self.refresh_ttl;

        let extra_map = if let Some(py_dict) = extra_payload {
            Python::with_gil(|py| {
                let dict = py_dict.as_ref(py);
                let mut extra_map = Map::new();
                for (key, value) in dict.iter() {
                    let key_str = key.extract::<String>()?;
                    let json_value = python_to_json(py, value)?;
                    extra_map.insert(key_str, json_value);
                }
                Ok::<Map<String, Value>, PyErr>(extra_map)
            })?
        } else {
            Map::new()
        };

        let claims = Claims {
            id: Some(user_id),
            exp: exp as usize,
            ttl: self.refresh_ttl as usize,
            token: "refresh".to_string(),
            jti: uuid.to_string(),
            extra: extra_map,
        };
        let algorithm = self.algorithm;

        let encoding_key = create_encoding_key(&self.secret, self.algorithm)?;
        
        let token = task::spawn_blocking(move || {
            encode(&Header::new(algorithm), &claims, &encoding_key)
        })
        .await
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

        Ok(token)
    }

    async fn decode_async_inner(&self, token: &str) -> PyResult<Py<PyDict>> {
        let mut validation = Validation::new(self.algorithm);
        validation.validate_exp = true;
        validation.required_spec_claims.clear();

        let decoding_key = create_decoding_key(&self.secret, self.algorithm)?;
        
        let token_owned = token.to_string();
        let token_data: TokenData<Claims> = task::spawn_blocking(move || {
            decode::<Claims>(&token_owned, &decoding_key, &validation)
        })
        .await
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

        Python::with_gil(|py| {
            let dict = PyDict::new(py);
            if let Some(id) = token_data.claims.id {
                dict.set_item("id", id)?;
            }
            dict.set_item("exp", token_data.claims.exp)?;
            dict.set_item("ttl", token_data.claims.ttl)?;
            dict.set_item("token", token_data.claims.token)?;
            dict.set_item("jti", token_data.claims.jti)?;

            for (key, value) in token_data.claims.extra {
                let py_value = json_to_python(py, &value)?;
                dict.set_item(key, py_value)?;
            }
            Ok(dict.into())
        })
    }
}

// Helper functions
async fn extra_map_from_py(extra_payload: Option<Py<PyDict>>) -> PyResult<Map<String, Value>> {
    if let Some(py_dict) = extra_payload {
        Python::with_gil(|py| {
            let dict = py_dict.as_ref(py);
            let mut extra_map = Map::new();
            for (key, value) in dict.iter() {
                let key_str = key.extract::<String>()?;
                let json_value = python_to_json(py, value)?;
                extra_map.insert(key_str, json_value);
            }
            Ok(extra_map)
        })
    } else {
        Ok(Map::new())
    }
}

fn create_encoding_key(secret: &str, algorithm: Algorithm) -> PyResult<EncodingKey> {
    match algorithm {
        Algorithm::HS256 | Algorithm::HS384 | Algorithm::HS512 => {
            Ok(EncodingKey::from_secret(secret.as_bytes()))
        }
        Algorithm::RS256
        | Algorithm::RS384
        | Algorithm::RS512
        | Algorithm::PS256
        | Algorithm::PS384
        | Algorithm::PS512 => EncodingKey::from_rsa_pem(secret.as_bytes())
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string())),
        Algorithm::ES256 | Algorithm::ES384 => EncodingKey::from_ec_pem(secret.as_bytes())
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string())),
        Algorithm::EdDSA => EncodingKey::from_ed_pem(secret.as_bytes())
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string())),
    }
}

fn create_decoding_key(secret: &str, algorithm: Algorithm) -> PyResult<DecodingKey> {
    match algorithm {
        Algorithm::HS256 | Algorithm::HS384 | Algorithm::HS512 => {
            Ok(DecodingKey::from_secret(secret.as_bytes()))
        }
        Algorithm::RS256
        | Algorithm::RS384
        | Algorithm::RS512
        | Algorithm::PS256
        | Algorithm::PS384
        | Algorithm::PS512 => DecodingKey::from_rsa_pem(secret.as_bytes())
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string())),
        Algorithm::ES256 | Algorithm::ES384 => DecodingKey::from_ec_pem(secret.as_bytes())
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string())),
        Algorithm::EdDSA => DecodingKey::from_ed_pem(secret.as_bytes())
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string())),
    }
}


#[pyfunction]
/// check if a token string looks like a valid JWT format
pub fn is_jwt_format(token: &str) -> bool {
    let parts: Vec<&str> = token.split('.').collect();
    parts.len() == 3 && parts.iter().all(|part| !part.is_empty())
}

#[pyfunction]
/// validate JWT format and return parts count
pub fn validate_jwt_format(token: &str) -> PyResult<bool> {
    if !is_jwt_format(token) {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Invalid JWT format: must have exactly 3 parts separated by dots",
        ));
    }
    Ok(true)
}


fn python_to_json(py: Python, obj: &PyAny) -> PyResult<Value> {
    if let Ok(s) = obj.extract::<String>() {
        Ok(Value::String(s))
    } else if let Ok(i) = obj.extract::<i64>() {
        Ok(Value::Number(i.into()))
    } else if let Ok(i) = obj.extract::<i128>() {
        if i <= i64::MAX as i128 && i >= i64::MIN as i128 {
            Ok(Value::Number((i as i64).into()))
        } else {
            Ok(Value::String(i.to_string()))
        }
    } else if let Ok(f) = obj.extract::<f64>() {
        if let Some(n) = serde_json::Number::from_f64(f) {
            Ok(Value::Number(n))
        } else {
            Ok(Value::Null)
        }
    } else if let Ok(b) = obj.extract::<bool>() {
        Ok(Value::Bool(b))
    } else if obj.is_none() {
        Ok(Value::Null)
    } else if let Ok(list) = obj.downcast::<PyList>() {
        let mut json_array = Vec::new();
        for item in list.iter() {
            json_array.push(python_to_json(py, item)?);
        }
        Ok(Value::Array(json_array))
    } else if let Ok(dict) = obj.downcast::<PyDict>() {
        let mut json_map = Map::new();
        for (key, value) in dict.iter() {
            let key_str = key.extract::<String>()?;
            json_map.insert(key_str, python_to_json(py, value)?);
        }
        Ok(Value::Object(json_map))
    } else {
        Ok(Value::String(obj.to_string()))
    }
}


fn json_to_python(py: Python, value: &Value) -> PyResult<PyObject> {
    match value {
        Value::Null => Ok(py.None()),
        Value::Bool(b) => Ok(b.into_py(py)),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(i.into_py(py))
            } else if let Some(u) = n.as_u64() {
                Ok(u.into_py(py))
            } else if let Some(f) = n.as_f64() {
                Ok(f.into_py(py))
            } else {
                Ok(n.to_string().into_py(py))
            }
        }
        Value::String(s) => Ok(s.into_py(py)),
        Value::Array(arr) => {
            let py_list = PyList::empty(py);
            for item in arr {
                py_list.append(json_to_python(py, item)?)?;
            }
            Ok(py_list.into_py(py))
        }
        Value::Object(obj) => {
            let py_dict = PyDict::new(py);
            for (key, value) in obj {
                py_dict.set_item(key, json_to_python(py, value)?)?;
            }
            Ok(py_dict.into_py(py))
        }
    }
}

#[pymodule]
fn ciphertoken(py: Python, m: &PyModule) -> PyResult<()> {
    //  Init Tokio runtime for async functions
    let mut builder = Builder::new_multi_thread();
    builder.enable_all();
    pyo3_asyncio::tokio::init(builder);

    m.add_class::<CipherToken>()?;
    m.add_function(wrap_pyfunction!(is_jwt_format, m)?)?;
    m.add_function(wrap_pyfunction!(validate_jwt_format, m)?)?;

    // ---------------- SECRET MODULE ----------------
    let secret_mod = secret::register_secret_module(py)?;
    m.add_submodule(secret_mod.as_ref(py))?;

    // ---------------- TIME MODULE ----------------
    let time_mod = time::register_time_module(py)?;
    m.add_submodule(time_mod.as_ref(py))?;


    // ---------------- UTILS MODULE ----------------
    let utils_mod = utils::register_utils_module(py)?;
    m.add_submodule(utils_mod.as_ref(py))?;

    // ---------------- ALGORITHMS MODULE ----------------
    let algo_mod = algorithms::register_algorithms_module(py)?;
    m.add_submodule(algo_mod.as_ref(py))?;

    Ok(())
}
