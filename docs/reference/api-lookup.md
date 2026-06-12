# API Lookup Reference

Quick searchable reference for all **CipherToken** classes, functions, constants, and async counterparts.

---

## Classes

| Class | Module | Description |
|-------|--------|-------------|
| [`CipherToken`](ciphertoken.md) | `ciphertoken` | Main token engine: sign, verify, decode, rotate, inspect |

---

## Synchronous Methods — `CipherToken`

| Method | Signature | Description |
|--------|-----------|-------------|
| `access` | `(payload: Optional[dict] = None) -> str` | Create access token |
| `refresh` | `(payload: Optional[dict] = None) -> str` | Create refresh token |
| `rotation` | `(refresh_token: str, payload: Optional[dict] = None) -> Tuple[str, str]` | Rotate refresh token |
| `decode` | `(token: str) -> dict` | Decode and validate token |
| `verify` | `(token: str) -> bool` | Check signature and expiry |
| `inspect` | `(token: str) -> dict` | Decode without strict validation |
| `remaining_time` | `(token: str) -> Optional[int]` | Seconds until expiry |
| `algorithm_name` | `() -> str` | Current algorithm name |

---

## Asynchronous Methods — `CipherToken`

| Method | Pattern | Sync Equivalent |
|--------|---------|-----------------|
| `access_async` | `(payload: Optional[dict] = None) -> str` | `access` |
| `refresh_async` | `(payload: Optional[dict] = None) -> str` | `refresh` |
| `rotation_async` | `(refresh_token: str, payload: Optional[dict] = None) -> Tuple[str, str]` | `rotation` |
| `decode_async` | `(token: str) -> dict` | `decode` |
| `verify_async` | `(token: str) -> bool` | `verify` |

---

## Helper Functions

| Function | Module | Signature | Description |
|----------|--------|-----------|-------------|
| `access` | `ciphertoken.jwt` | `(token, payload=None) -> str` | Create access token via jwt module |
| `refresh` | `ciphertoken.jwt` | `(token, payload=None) -> str` | Create refresh token via jwt module |
| `rotation` | `ciphertoken.jwt` | `(token, refresh_token, payload=None) -> Tuple[str, str]` | Rotate via jwt module |
| `access_async` | `ciphertoken.jwt` | `(token, payload=None) -> str` | Async access via jwt module |
| `refresh_async` | `ciphertoken.jwt` | `(token, payload=None) -> str` | Async refresh via jwt module |
| `rotation_async` | `ciphertoken.jwt` | `(token, refresh_token, payload=None) -> Tuple[str, str]` | Async rotation via jwt module |
| `is_jwt_format` | `ciphertoken` | `(token: str) -> bool` | Validate JWT format |
| `validate_jwt_format` | `ciphertoken` | `(token: str) -> bool` | Validate JWT format or raise `ValueError` |
| `secret_key` | `ciphertoken.secret` | `() -> str` | Generate HMAC secret (32 bytes) |
| `secret_key_with_size` | `ciphertoken.secret` | `(size: int) -> str` | Generate HMAC secret with custom size |
| `generate_hmac_secret` | `ciphertoken.secret` | `(size: int) -> str` | Generate HMAC secret |
| `generate_hmac_secret_async` | `ciphertoken.secret` | `(size: int) -> str` | Async HMAC secret generation |
| `generate_rsa_keypair` | `ciphertoken.secret` | `(bits: Optional[int] = 2048) -> Tuple[str, str]` | Generate RSA key pair in PEM |

---

## Time Utilities

| Function | Signature | Result |
|----------|-----------|--------|
| `now` | `time.now() -> int` | Current UNIX timestamp |
| `seconds` | `time.seconds(n: int) -> int` | Returns `n` |
| `minutes` | `time.minutes(n: int) -> int` | `n * 60` |
| `hours` | `time.hours(n: int) -> int` | `n * 3600` |
| `days` | `time.days(n: int) -> int` | `n * 86400` |
| `weeks` | `time.weeks(n: int) -> int` | `n * 604800` |

---

## Constants

| Constant | Value | Module |
|----------|-------|--------|
| `TOKEN_ACCESS` | `"access"` | `ciphertoken.utils`, `ciphertoken.jwt` |
| `TOKEN_REFRESH` | `"refresh"` | `ciphertoken.utils`, `ciphertoken.jwt` |
| `DEFAULT_SECRET_SIZE` | `32` | `ciphertoken.utils` |
| `MIN_SECRET_SIZE` | `16` | `ciphertoken.utils` |

---

## Algorithm Constants

| Algorithm | Family |
|-----------|--------|
| `HS256`, `HS384`, `HS512` | HMAC |
| `RS256`, `RS384`, `RS512` | RSA |
| `ES256`, `ES384` | ECDSA |
| `PS256`, `PS384`, `PS512` | RSA-PSS |
| `EDDSA` | Edwards Curve |
