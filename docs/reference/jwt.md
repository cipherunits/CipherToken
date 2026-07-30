---
title: jwt Module — Convenience Functions API
description: High-level JWT convenience functions for access, refresh, and rotation operations — sync and async. Built with Rust for Python.
keywords: jwt module python, access token function, refresh token function, token rotation python
image: https://cipherunits.github.io/CipherToken/logo.png
---

# API Reference — jwt

The `jwt` module provides convenient top-level functions for common JWT operations. These are thin wrappers around `CipherToken` methods, offering two equivalent ways to call them.

---

## Synchronous Functions

### `access(token: CipherToken, payload: Optional[Dict[str, Any]] = None) -> str`

Generate an access token. Equivalent to `token.access(payload)`.

```python
# Method 1: Module function
from ciphertoken.jwt import access
access_token = access(ct, payload={"user_id": 42})

# Method 2: CipherToken method (identical behavior)
access_token = ct.access(payload={"user_id": 42})

# Method 3: Direct create_token call
access_token = ct.create_token(ttl_time=3600, token_type="access", payload={"user_id": 42})
```

---

### `refresh(token: CipherToken, payload: Optional[Dict[str, Any]] = None) -> str`

Generate a refresh token. Equivalent to `token.refresh(payload)`.

```python
# Method 1: Module function
from ciphertoken.jwt import refresh
refresh_token = refresh(ct, payload={"user_id": 42})

# Method 2: CipherToken method
refresh_token = ct.refresh(payload={"user_id": 42})

# Method 3: Direct create_token call
refresh_token = ct.create_token(ttl_time=604800, token_type="refresh", payload={"user_id": 42})
```

---

### `rotation(token: CipherToken, refresh_token: str, payload: Optional[Dict[str, Any]] = None) -> Tuple[str, str]`

Rotate a refresh token. Returns a new access and refresh token pair. Equivalent to `token.rotation(refresh_token, payload)`.

```python
# Method 1: Module function
from ciphertoken.jwt import rotation
new_access, new_refresh = rotation(ct, old_refresh_token, payload={"user_id": 42})

# Method 2: CipherToken method
new_access, new_refresh = ct.rotation(old_refresh_token, payload={"user_id": 42})
```

**Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `token` | `CipherToken` | _(required)_ | CipherToken instance |
| `refresh_token` | `str` | _(required)_ | Existing refresh token string |
| `payload` | `Optional[Dict[str, Any]]` | `None` | New payload for rotated tokens |

**Returns:** `Tuple[str, str]` - (new_access_token, new_refresh_token)

**Raises:**
- `ValueError`: If provided token is not a refresh token

---

## Asynchronous Functions

### `access_async(token: CipherToken, payload: Optional[Dict[str, Any]] = None) -> str`

Async version of `access`. Equivalent to `await token.access_async(payload)`.

```python
# Method 1: Module function
from ciphertoken.jwt import access_async
access_token = await access_async(ct, payload={"user_id": 42})

# Method 2: CipherToken method
access_token = await ct.access_async(payload={"user_id": 42})
```

---

### `refresh_async(token: CipherToken, payload: Optional[Dict[str, Any]] = None) -> str`

Async version of `refresh`. Equivalent to `await token.refresh_async(payload)`.

```python
# Method 1: Module function
from ciphertoken.jwt import refresh_async
refresh_token = await refresh_async(ct, payload={"user_id": 42})

# Method 2: CipherToken method
refresh_token = await ct.refresh_async(payload={"user_id": 42})
```

---

### `rotation_async(token: CipherToken, refresh_token: str, payload: Optional[Dict[str, Any]] = None) -> Tuple[str, str]`

Async version of `rotation`. Equivalent to `await token.rotation_async(refresh_token, payload)`.

```python
# Method 1: Module function
from ciphertoken.jwt import rotation_async
new_access, new_refresh = await rotation_async(ct, old_refresh_token)

# Method 2: CipherToken method
new_access, new_refresh = await ct.rotation_async(old_refresh_token)
```

---

## Constants

| Constant | Value | Description |
|----------|-------|-------------|
| `TOKEN_ACCESS` | `"access"` | Token type string for access tokens |
| `TOKEN_REFRESH` | `"refresh"` | Token type string for refresh tokens |

These are exported from both `ciphertoken.jwt` and `ciphertoken.utils`.

---

## Complete Workflow Examples

### Synchronous

```python
from ciphertoken import CipherToken
from ciphertoken.jwt import access, refresh, rotation
from ciphertoken.algorithms import HS256
from ciphertoken.time import minutes, days

# Initialize
ct = CipherToken(
    secret="your-secret-key",
    algorithm=HS256,
    access_ttl=minutes(10),
    refresh_ttl=days(7),
)

# Create tokens (module style)
access_token = access(ct, payload={"sub": "user@example.com"})
refresh_token = refresh(ct, payload={"sub": "user@example.com"})

# Verify
print(ct.verify(access_token))   # True
print(ct.verify(refresh_token))  # True

# Decode
claims = ct.decode(access_token)
print(claims["token_type"])  # "access"
print(claims["exp"])         # UNIX timestamp

# Rotate
new_access, new_refresh = rotation(ct, refresh_token, payload={"sub": "user@example.com"})

# Verify new tokens
print(ct.verify(new_access))   # True
print(ct.verify(new_refresh))  # True
```

### Asynchronous

```python
import asyncio
from ciphertoken import CipherToken
from ciphertoken.jwt import access_async, refresh_async, rotation_async
from ciphertoken.algorithms import HS256
from ciphertoken.time import minutes, days

async def main():
    ct = CipherToken(
        secret="your-secret-key",
        algorithm=HS256,
        access_ttl=minutes(10),
        refresh_ttl=days(7),
    )

    # Create tokens
    access_token = await access_async(ct, payload={"sub": "user@example.com"})
    refresh_token = await refresh_async(ct, payload={"sub": "user@example.com"})

    # Verify
    print(await ct.verify_async(access_token))   # True

    # Decode
    claims = await ct.decode_async(access_token)
    print(claims["token_type"])  # "access"

    # Rotate
    new_access, new_refresh = await rotation_async(ct, refresh_token)

asyncio.run(main())
```

---

## Error Handling

```python
from ciphertoken.jwt import rotation

try:
    # Will raise ValueError if token is not a refresh token
    new_access, new_refresh = rotation(ct, access_token)
except ValueError as e:
    print(e)  # "Only refresh tokens can be used for rotation"

try:
    # Will raise ValueError for invalid tokens
    ct.decode("invalid.token.here")
except ValueError as e:
    print(e)  # JWT error message
```

---

## Quick Lookup

➡️ [API Index](api-lookup.md) — Searchable method and constant reference