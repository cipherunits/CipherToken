---
title: CipherToken Class — API Reference
description: Complete API reference for the CipherToken class. Methods for signing, verification, decoding, rotation, and expiry inspection.
keywords: ciphertoken class, jwt api python, token verify decode rotate, ciphertoken methods
image: https://cipherunits.github.io/CipherToken/logo.png
---

# API Reference — CipherToken

The `CipherToken` class is the core token engine. It handles signing, verification, decoding, rotation, and expiry inspection for both **access** and **refresh** tokens.

---

## Constructor

```python
from ciphertoken import CipherToken

token = CipherToken(
    secret: str,
    algorithm: str = "HS256",
    access_ttl: int = 3600,
    refresh_ttl: int = 604800,
)
```

### Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `secret` | `str` | _(required)_ | Signing key: HMAC secret string, or RSA/ECDSA private key in PEM format |
| `algorithm` | `str` | `"HS256"` | Algorithm constant from `ciphertoken.algorithms` (HS256, HS384, HS512, RS256, RS384, RS512, ES256, ES384, PS256, PS384, PS512, EDDSA) |
| `access_ttl` | `int` | `3600` | Access token lifetime in seconds (1 hour) |
| `refresh_ttl` | `int` | `604800` | Refresh token lifetime in seconds (7 days) |

---

## Properties

| Property | Type | Description |
|----------|------|-------------|
| `secret` | `str` | Masked secret (first 8 chars + `...`) for security; returns `"***"` if secret is shorter than 8 characters |
| `access_ttl` | `int` | Access token TTL in seconds |
| `refresh_ttl` | `int` | Refresh token TTL in seconds |

---

## Token Creation Methods

### `access(payload: Optional[Dict[str, Any]] = None) -> str`

Create an access token. Can be called in multiple ways:

```python
# Method 1: Using CipherToken instance method
ct = CipherToken(secret="my-secret", algorithm="HS256")
access_token = ct.access(payload={"user_id": 42})

# Method 2: Using jwt module function
from ciphertoken.jwt import access
access_token = access(ct, payload={"user_id": 42})

# Method 3: Using create_token directly (same as access)
access_token = ct.create_token(ttl_time=3600, token_type="access", payload={"user_id": 42})
```

---

### `refresh(payload: Optional[Dict[str, Any]] = None) -> str`

Create a refresh token. Can be called in multiple ways:

```python
# Method 1: Using CipherToken instance method
refresh_token = ct.refresh(payload={"user_id": 42})

# Method 2: Using jwt module function
from ciphertoken.jwt import refresh
refresh_token = refresh(ct, payload={"user_id": 42})

# Method 3: Using create_token directly
refresh_token = ct.create_token(ttl_time=604800, token_type="refresh", payload={"user_id": 42})
```

---

### `payload(payload: Optional[Dict[str, Any]] = None) -> str`

Create a generic access token (alias for `access`). Same behavior as `access()`.

---

### `create_token(ttl_time: int = 3153600000, token_type: str = "jwt", payload: Optional[Dict[str, Any]] = None) -> str`

Core token creation method. Creates tokens with custom TTL and type.

**Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `ttl_time` | `int` | `3153600000` | Time-to-live in seconds (max ~100 years) |
| `token_type` | `str` | `"jwt"` | Token type identifier (`"access"` or `"refresh"`) |
| `payload` | `Optional[Dict[str, Any]]` | `None` | Custom claims to include in token |

```python
# Custom TTL token
custom_token = ct.create_token(ttl_time=7200, token_type="access", payload={"role": "admin"})
```

---

## Token Verification Methods

### `verify(token: str) -> bool`

Validate token signature and expiry. Returns `True` if valid, `False` if invalid.

```python
# Valid token
ct.verify(access_token)  # True

# Invalid or expired token
ct.verify("invalid.token.here")  # False
```

---

### `decode(token: str) -> Dict[str, Any]`

Decode and validate a token. Returns full claims as a dictionary. Raises `ValueError` on invalid tokens.

**Returns:**

| Key | Type | Description |
|-----|------|-------------|
| `exp` | `int` | UNIX expiry timestamp |
| `ttl` | `int` | Time-to-live in seconds |
| `token_type` | `str` | `"access"` or `"refresh"` |
| `jti` | `str` | UUID v4 identifier |
| _other keys_ | varies | User-provided payload claims |

```python
claims = ct.decode(access_token)
# {
#     'exp': 1716000000,
#     'ttl': 3600,
#     'token_type': 'access',
#     'jti': '550e8400-e29b-41d4-a716-446655440000',
#     'user_id': 42,
#     'role': 'admin'
# }
```

---

### `inspect(token: str) -> Dict[str, Any]`

Decode without strict signature or expiry validation. Useful for debugging expired tokens.

```python
# Inspect an expired token without error
claims = ct.inspect(expired_token)
```

---

### `remaining_time(token: str) -> Optional[int]`

Get seconds until token expiry. Returns `None` if the token has no expiry claim.

```python
remaining = ct.remaining_time(access_token)  # e.g., 3599
```

---

## Token Rotation Methods

### `rotation(refresh_token: str, payload: Optional[Dict[str, Any]] = None) -> Tuple[str, str]`

Rotate a refresh token into a new access + refresh pair. Raises `ValueError` if the provided token is not a refresh token.

```python
# Method 1: Instance method
new_access, new_refresh = ct.rotation(old_refresh_token, payload={"user_id": 42})

# Method 2: Module function
from ciphertoken.jwt import rotation
new_access, new_refresh = rotation(ct, old_refresh_token, payload={"user_id": 42})
```

**Raises:**
- `ValueError`: If token_type is not `"refresh"`

---

### `rotation_async(refresh_token: str, payload: Optional[Dict[str, Any]] = None) -> Tuple[str, str]`

Async version of rotation. Use in async contexts.

```python
# Instance method
new_access, new_refresh = await ct.rotation_async(old_refresh_token)

# Module function
from ciphertoken.jwt import rotation_async
new_access, new_refresh = await rotation_async(ct, old_refresh_token)
```

---

## Algorithm Methods

### `algorithm_name() -> str`

Get the current algorithm name as a string.

```python
ct.algorithm_name()  # "HS256", "RS256", etc.
```

---

## Synchronous Methods Summary

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

## Asynchronous Methods Summary

| Method | Signature | Description |
|--------|-----------|-------------|
| `access_async` | `(payload: Optional[dict] = None) -> str` | Async access token |
| `refresh_async` | `(payload: Optional[dict] = None) -> str` | Async refresh token |
| `decode_async` | `(token: str) -> dict` | Async decode and validate |
| `verify_async` | `(token: str) -> bool` | Async verify signature |
| `rotation_async` | `(refresh_token: str, payload: Optional[dict] = None) -> Tuple[str, str]` | Async rotation |

---

## Complete Workflow Example

```python
from ciphertoken import CipherToken
from ciphertoken.jwt import access, refresh, rotation
from ciphertoken.algorithms import HS256

# Initialize once
ct = CipherToken(
    secret="your-strong-secret-key",
    algorithm=HS256,
    access_ttl=3600,
    refresh_ttl=604800,
)

# Create tokens (3 equivalent styles — each call produces a token
# with a unique jti, so the strings themselves differ)
access_token = ct.access(payload={"user_id": 42})
access_token = access(ct, payload={"user_id": 42})
access_token = ct.create_token(ttl_time=3600, token_type="access", payload={"user_id": 42})

# Verify
ct.verify(access_token)   # True

# Decode
claims = ct.decode(access_token)
# {'exp': 1716000000, 'ttl': 3600, 'token_type': 'access', 'jti': '...', 'user_id': 42}

# Rotate
refresh_token = refresh(ct, payload={"user_id": 42})
new_access, new_refresh = ct.rotation(refresh_token)
```

---

## Async Workflow Example

```python
import asyncio
from ciphertoken import CipherToken
from ciphertoken.jwt import access_async, refresh_async, rotation_async

async def main():
    ct = CipherToken(secret="my-secret", algorithm="HS256")
    
    # Async token creation (2 ways)
    access_1 = await ct.access_async(payload={"user_id": 1})
    access_2 = await access_async(ct, payload={"user_id": 1})

    # Refresh token
    refresh_1 = await ct.refresh_async(payload={"user_id": 1})
    refresh_2 = await refresh_async(ct, payload={"user_id": 1})

    # Rotate
    new_access, new_refresh = await ct.rotation_async(refresh_1)

asyncio.run(main())
```

---

## Claims Structure

Every token contains four automatic claims, and your payload keys are merged alongside them at the **top level** (there is no nested `"payload"` key):

| Claim | Type | Description |
|-------|------|-------------|
| `exp` | `int` | UNIX expiry timestamp |
| `ttl` | `int` | Time-to-live in seconds when token was created |
| `token_type` | `str` | `"access"` or `"refresh"` |
| `jti` | `str` | UUID v4 identifier |
| _your keys_ | varies | User-provided payload claims, flattened |

```python
claims = ct.decode(ct.access(payload={"user_id": 42}))
claims["user_id"]  # 42 — top level, not claims["payload"]["user_id"]
```

---

## Quick Lookup

➡️ [API Index](api-lookup.md) — Searchable method and constant reference