# API Reference — CipherToken

The `CipherToken` class is the core token engine. It handles signing, verification, decoding, rotation, and expiry inspection for both **access** and **refresh** tokens.

---

## Constructor

```python
from ciphertoken import CipherToken

token = CipherToken(
    secret: str,
    algorithm: str,
    access_ttl: int,
    refresh_ttl: int,
)
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `secret` | `str` | Signing key (HMAC secret or RSA/ECDSA private key in PEM) |
| `algorithm` | `str` | Algorithm constant from `ciphertoken.algorithms` |
| `access_ttl` | `int` | Access token lifetime in seconds |
| `refresh_ttl` | `int` | Refresh token lifetime in seconds |

---

## Properties

| Property | Type | Description |
|----------|------|-------------|
| `secret` | `str` | Masked secret (first 8 chars + `...`) |
| `access_ttl` | `int` | Access token TTL in seconds |
| `refresh_ttl` | `int` | Refresh token TTL in seconds |

---

## Synchronous Methods

<div class="grid cards" markdown>

```python
payload(payload=None) -> str
```

Create a generic access token.

---

```python
access(payload=None) -> str
```

Create an access token.

---

```python
refresh(payload=None) -> str
```

Create a refresh token.

---

```python
decode(token) -> dict
```

Decode and validate a token. Returns full claims. Raises `ValueError` if invalid.

---

```python
verify(token) -> bool
```

Validate signature and expiry.

---

```python
rotation(refresh_token, payload=None) -> Tuple[str, str]
```

Rotate a refresh token into a new access + refresh pair.

---

```python
inspect(token) -> dict
```

Decode without strict signature or expiry validation. For debugging.

---

```python
remaining_time(token) -> Optional[int]
```

Seconds until token expiry, or `None`.

---

```python
algorithm_name() -> str
```

Current algorithm string (e.g., `"HS256"`).

</div>

---

## Asynchronous Methods

<div class="grid cards" markdown>

```python
access_async(payload=None) -> str
```

Async access token generation.

---

```python
refresh_async(payload=None) -> str
```

Async refresh token generation.

---

```python
decode_async(token) -> dict
```

Async decode and validate.

---

```python
verify_async(token) -> bool
```

Async verify signature and expiry.

---

```python
rotation_async(refresh_token, payload=None) -> Tuple[str, str]
```

Async token rotation.

</div>

---

## Helper Functions

| Function | Signature | Description |
|----------|-----------|-------------|
| `is_jwt_format` | `(token: str) -> bool` | Fast format check — 3 dot-separated parts |
| `validate_jwt_format` | `(token: str) -> bool` | Same as above, raises `ValueError` on invalid |

---

## Claims Structure

Every token contains:

| Claim | Type | Description |
|-------|------|-------------|
| `exp` | `int` | UNIX expiry timestamp |
| `ttl` | `int` | Time-to-live in seconds |
| `token` | `str` | `"access"` or `"refresh"` |
| `jti` | `str` | UUID v4 identifier |
| `payload` | `dict` | User-provided claims |

---

## Quick Lookup

➡️ [API Index](api-lookup.md) — Searchable method and constant reference
