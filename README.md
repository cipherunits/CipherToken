<img src="https://raw.githubusercontent.com/cipher-unit/CipherToken/refs/heads/main/logo.png" width=100>

# CipherToken

**CipherToken** is a modern, high-performance Python library for cryptography and JWT (JSON Web Token) management.  
It is implemented in **Rust** using **PyO3**, providing both speed and security for generating, encoding, decoding, and verifying tokens.  

The library is designed to be modular, organized into **submodules** for secret management, time utilities, algorithms, and token handling.  
It supports both **synchronous** and **asynchronous** operations

---

## Library Overview

CipherToken is structured into the following main parts:

### 1. `secret` Module
Handles key generation for symmetric (HMAC) and asymmetric (RSA) algorithms:

- `secret_key()` – Generate a random HMAC key (default 32 bytes)
- `secret_key_with_size(size: int)` – Generate HMAC key with a custom size
- `generate_hmac_secret(size: int)` – Generate an HMAC secret key
- `generate_hmac_secret_async(size: int)` – Async version of HMAC key generation
- `generate_rsa_keypair(bits: Optional[int] = 2048)` – Generate RSA private/public key pair

---

### 2. `time` Module
Provides utility functions for time conversion and timestamps:

- `now()` – Current UNIX timestamp
- `seconds(n: int)` – Convert n seconds to seconds
- `minutes(n: int)` – Convert n minutes to seconds
- `hours(n: int)` – Convert n hours to seconds
- `days(n: int)` – Convert n days to seconds
- `weeks(n: int)` – Convert n weeks to seconds

---

### 3. `utils` Module
Contains useful constants:

- `DEFAULT_SECRET_SIZE` – Default key size
- `MIN_SECRET_SIZE` – Minimum allowed key size
- `TOKEN_ACCESS` – Access token type
- `TOKEN_REFRESH` – Refresh token type

---

### 4. `algorithms` Module
Defines algorithm constants for token signing:

- HMAC: `HS256`, `HS384`, `HS512`
- RSA: `RS256`, `RS384`, `RS512`
- ECDSA: `ES256`, `ES384`
- RSA-PSS: `PS256`, `PS384`, `PS512`
- Edwards Curve: `EDDSA`

---

### 5. `CipherToken` Class
Main class for managing JWTs:

- **Constructor:**
```python
from ciphertoken import CipherToken

CipherToken(secret: str, algorithm: str, access_ttl: int, refresh_ttl: int)
```

### Synchronous methods:
```python
access(user_id: int, extra_payload: Optional[dict] = None) -> str
refresh(user_id: int, extra_payload: Optional[dict] = None) -> str
decode(token: str) -> dict
verify(token: str) -> bool
rotation(refresh_token: str, extra_payload: Optional[dict] = None) -> Tuple[str, str]
inspect(token: str) -> dict
remaining_time(token: str) -> Optional[int]
algorithm_name() -> str
```

### Asynchronous methods:
```python
await access_async(user_id: int, extra_payload: Optional[dict] = None) -> str
await refresh_async(user_id: int, extra_payload: Optional[dict] = None) -> str
await decode_async(token: str) -> dict
await verify_async(token: str) -> bool
await rotation_async(refresh_token: str, extra_payload: Optional[dict] = None) -> Tuple[str, str]
```

### Helper Functions:

`is_jwt_format(token: str) – Check if a string is a valid JWT`

`validate_jwt_format(token: str) – Validate JWT structure`

## Installation

CipherToken is available via PyPI. Install it with:

`pip install ciphertoken`

---

### Usage Example

```python
from ciphertoken.secret import secret_key
from ciphertoken import CipherToken
from ciphertoken.time import minutes, days
from ciphertoken.utils import TOKEN_REFRESH
from ciphertoken.algorithms import HS256

# Generate a secret key
key = secret_key()

# Create a CipherToken instance
token = CipherToken(secret=key, algorithm=HS256, access_ttl=minutes(10), refresh_ttl=days(7))

# Generate access and refresh tokens
access_token = token.access(user_id=123)
refresh_token = token.refresh(user_id=123)

# Verify and decode tokens
payload = token.decode(access_token)
is_valid = token.verify(access_token)

# Async usage
import asyncio

async def generate_async_token():
    access_token = await token.access_async(user_id=123)
    payload = await token.decode_async(access_token)
    return payload

asyncio.run(generate_async_token())

```

### Implementation Notes

- Built in Rust using PyO3 for maximum speed and memory safety

- Supports synchronous and asynchronous workflows

- Fully modular: secret, time, utils, algorithms, and CipherToken class

- Secure key handling with HMAC and RSA support

- Easily extendable for additional algorithms


### Summary

CipherToken is a modern, Rust-backed Python library for authentication and JWT management.
It is ideal for web APIs, microservices, or any system that requires secure token generation, verification, and rotation.