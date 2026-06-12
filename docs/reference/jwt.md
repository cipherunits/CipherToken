# API Reference — jwt

The `jwt` module provides convenient top-level functions for common JWT flows. These are thin wrappers around the `CipherToken` class methods.

## Synchronous Functions

### `access(token: CipherToken, payload: Optional[dict] = None) -> str`

Generate an access token from a `CipherToken` instance.

```python
from ciphertoken.jwt import access

access_token = access(token, payload={"user_id": 42})
```

---

### `refresh(token: CipherToken, payload: Optional[dict] = None) -> str`

Generate a refresh token from a `CipherToken` instance.

```python
from ciphertoken.jwt import refresh

refresh_token = refresh(token, payload={"user_id": 42})
```

---

### `rotation(token: CipherToken, refresh_token: str, payload: Optional[dict] = None) -> Tuple[str, str]`

Rotate a refresh token. Returns a new access and refresh token pair.

```python
from ciphertoken.jwt import rotation

new_access, new_refresh = rotation(token, refresh_token, payload={"user_id": 42})
```

---

## Asynchronous Functions

### `access_async(token: CipherToken, payload: Optional[dict] = None) -> str`

Async version of `access`.

```python
from ciphertoken.jwt import access_async

access_token = await access_async(token, payload={"user_id": 42})
```

---

### `refresh_async(token: CipherToken, payload: Optional[dict] = None) -> str`

Async version of `refresh`.

```python
from ciphertoken.jwt import refresh_async

refresh_token = await refresh_async(token, payload={"user_id": 42})
```

---

### `rotation_async(token: CipherToken, refresh_token: str, payload: Optional[dict] = None) -> Tuple[str, str]`

Async version of `rotation`.

```python
from ciphertoken.jwt import rotation_async

new_access, new_refresh = await rotation_async(token, refresh_token)
```

---

## Constants

| Constant | Value | Description |
|----------|-------|-------------|
| `TOKEN_ACCESS` | `"access"` | Token type string for access tokens |
| `TOKEN_REFRESH` | `"refresh"` | Token type string for refresh tokens |

---

## Complete Workflow Example

```python
from ciphertoken import CipherToken
from ciphertoken.jwt import access, refresh, rotation
from ciphertoken.algorithms import HS256
from ciphertoken.time import minutes, days

# Initialize
token = CipherToken(
    secret="your-secret-key",
    algorithm=HS256,
    access_ttl=minutes(10),
    refresh_ttl=days(7),
)

# Step 1: Create tokens
access_token = access(token, payload={"sub": "user@example.com"})
refresh_token = refresh(token, payload={"sub": "user@example.com"})

# Step 2: Verify
print(token.verify(access_token))      # True
print(token.verify(refresh_token))    # True

# Step 3: Decode
print(token.decode(access_token))

# Step 4: Rotate
new_access, new_refresh = rotation(token, refresh_token, payload={"sub": "user@example.com"})

# Step 5: Verify new tokens
print(token.verify(new_access))       # True
print(token.verify(new_refresh))     # True
```
