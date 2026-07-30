---
title: JWT Security Best Practices
description: Security best practices for JWT tokens in Python — secret management, key rotation, algorithm selection, token expiry, and payload validation.
keywords: jwt security best practices, jwt secret management, jwt key rotation, secure jwt python, token expiry
image: https://cipherunits.github.io/CipherToken/logo.png
---

# Advanced Guide — Security

Follow these security best practices to keep your token infrastructure safe. Part of **[Cipher-Unit](https://cipherunit.xyz/)**.

---

## Table of Contents

- [Secret Management](#secret-management)
- [Key Rotation](#key-rotation)
- [Algorithm Selection](#algorithm-selection)
- [Token Expiry](#token-expiry)
- [Input Validation](#input-validation)
- [Never Log Secrets](#never-log-secrets)

---

## Secret Management

### HMAC Secrets

- Always use secrets with a minimum of **32 bytes** (256 bits).
- Use `secret_key_with_size` instead of hardcoding.
- Store secrets in environment variables or secret managers.

```python
import os
from ciphertoken import CipherToken
from ciphertoken.algorithms import HS256
from ciphertoken.time import minutes, days

# Generate a 64-byte secret once (e.g., secret_key_with_size(64)),
# store it securely, and load it from the environment at runtime
SECRET = os.environ["CIPHERTOKEN_SECRET"]

token = CipherToken(
    secret=SECRET,
    algorithm=HS256,
    access_ttl=minutes(10),
    refresh_ttl=days(7),
)
```

### RSA Key Storage

- Store private keys in a secure vault (e.g., **HashiCorp Vault**, **AWS Secrets Manager**, or **Azure Key Vault**).
- Never commit private keys to version control.
- Use **4096-bit** keys for production systems requiring maximum security.

---

## Key Rotation

Rotate secrets periodically without downtime:

```python
import os
from ciphertoken import CipherToken
from ciphertoken.algorithms import HS256
from ciphertoken.time import minutes, days

# Verify both old and new tokens during a rotation window
validators = [
    CipherToken(secret=os.environ["NEW_SECRET"], algorithm=HS256,
                access_ttl=minutes(10), refresh_ttl=days(7)),
    CipherToken(secret=os.environ["CURRENT_SECRET"], algorithm=HS256,
                access_ttl=minutes(10), refresh_ttl=days(7)),
]

def verify_token(token_str):
    for validator in validators:
        try:
            if validator.verify(token_str):
                return validator.decode(token_str)
        except Exception:
            continue
    raise ValueError("Invalid token")

# After the grace period, retire the old secret
```

For rotating **tokens** (as opposed to secrets), use the built-in rotation:

```python
new_access, new_refresh = validators[0].rotation(refresh_token)
```

---

## Algorithm Selection

| Scenario | Recommendation |
|----------|---------------|
| Internal microservices with shared secret | `HS256` or `HS384` |
| Public API with third-party consumers | `RS256`, `ES256`, or `EDDSA` |
| Regulatory/compliance requirements | `PS256`, `PS384`, or `PS512` |
| Legacy system compatibility | Check the JWT consumer's supported algorithms first |

!!! danger "The `none` algorithm is not supported — by design"
    The unsigned `none` algorithm is a well-known JWT attack vector: it means no signature at all. CipherToken does not expose it, so tokens are always cryptographically signed.

---

## Token Expiry

- Keep access tokens short-lived (5–15 minutes).
- Use refresh tokens to obtain new access tokens.
- Set refresh token TTL to your session expiration policy (hours to days).

```python
from ciphertoken.time import minutes, days

# Recommended settings
token = CipherToken(
    secret=SECRET,
    algorithm=HS256,
    access_ttl=minutes(10),
    refresh_ttl=days(7),
)
```

---

## Input Validation

Always validate incoming payloads before trusting them:

```python
claims = token.decode(access_token)

# Payload keys are flattened to the top level of the claims dict
user_id = claims.get("user_id")
if not isinstance(user_id, int):
    raise ValueError("Invalid payload")

# Validate roles
if "admin" not in claims.get("roles", []):
    raise PermissionError("Forbidden")
```

---

## Never Log Secrets

Never log `token.secret` or raw key material. CipherToken masks the secret property, but it is your responsibility to avoid exposing secrets in:

- Log files
- Exception messages
- Monitoring/observability tools
- Debuggers during development

!!! danger "Leaked secrets are irrecoverable"
    Even partial key exposure can allow attackers to forge tokens. Treat secrets as you would a database password.
