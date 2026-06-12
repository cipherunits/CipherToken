# Quick Start

Generate and manage JWTs in under 2 minutes with **CipherToken**.

---

## 1. Initialize

```python
from ciphertoken import CipherToken
from ciphertoken.algorithms import HS256
from ciphertoken.time import minutes, days

token = CipherToken(
    secret="your-strong-secret-key",
    algorithm=HS256,
    access_ttl=minutes(10),
    refresh_ttl=days(7),
)
```

!!! warning "Never hardcode secrets"
    Use environment variables or a secret manager in production. See the [Security guide](../advanced/security.md).

---

## 2. Generate Tokens

```python
# Access token — used to authenticate API requests
access_token = token.access(payload={"user_id": 42, "role": "admin"})

# Refresh token — used to obtain a new access token
refresh_token = token.refresh(payload={"user_id": 42})
```

---

## 3. Verify and Decode

```python
# Quick validation
is_valid = token.verify(access_token)

# Full claims retrieval
claims = token.decode(access_token)
# {
#   'payload': {'user_id': 42, 'role': 'admin'},
#   'exp': 1716000000,
#   'ttl': 600,
#   'token': 'access',
#   'jti': '550e8400-e29b-41d4-a716-446655440000'
# }
```

---

## 4. Rotate Tokens

```python
# Exchange a refresh token for a new pair
new_access, new_refresh = token.rotation(refresh_token, payload={"user_id": 42})
```

---

## 5. Inspect and Check Expiry

```python
# Inspect without strict validation (debugging)
claims = token.inspect(access_token)

# Remaining seconds until expiry
remaining = token.remaining_time(access_token)
```

---

## Generate Keys

### HMAC Secret

```python
from ciphertoken.secret import secret_key, secret_key_with_size

secret = secret_key()                   # 32 bytes (default)
secret = secret_key_with_size(64)       # custom size
```

### RSA Key Pair

```python
from ciphertoken.secret import generate_rsa_keypair
from ciphertoken.algorithms import RS256

private_key, public_key = generate_rsa_keypair(bits=2048)

token = CipherToken(
    secret=private_key,
    algorithm=RS256,
    access_ttl=minutes(15),
    refresh_ttl=days(7),
)
```

---

## Time Utilities

```python
from ciphertoken.time import now, minutes, hours, days, weeks

minutes(10)   # 600
hours(2)      # 7200
days(7)       # 604800
```

---

## Supported Algorithms

| Family      | Algorithms                          | Key               |
|-------------|-------------------------------------|-------------------|
| HMAC        | `HS256`, `HS384`, `HS512`           | Shared secret     |
| RSA         | `RS256`, `RS384`, `RS512`           | Key pair          |
| ECDSA       | `ES256`, `ES384`                    | Key pair          |
| RSA-PSS     | `PS256`, `PS384`, `PS512`           | Key pair          |
| Edwards     | `EDDSA`                             | Ed25519 key pair  |

---

➡️ [API Reference](../reference/index.md) — Explore every module and method.
