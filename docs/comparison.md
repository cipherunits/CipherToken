---
title: CipherToken vs PyJWT vs python-jose
description: Compare CipherToken with PyJWT and python-jose. Performance, async support, key generation, token rotation, and how to migrate from PyJWT.
keywords: pyjwt alternative, python-jose alternative, pyjwt vs ciphertoken, jwt library comparison python, migrate from pyjwt
image: https://cipherunits.github.io/CipherToken/logo.png
---

# CipherToken vs PyJWT vs python-jose

Choosing a JWT library for Python? This page compares **CipherToken** with the two most popular options — **PyJWT** and **python-jose** — across performance, features, and developer experience.

---

## Feature Comparison

| Feature | CipherToken | PyJWT | python-jose |
|---|---|---|---|
| **Implementation** | Rust (compiled) | Pure Python | Pure Python |
| **Token creation speed** | ~587k ops/sec | ~102k ops/sec | ~104k ops/sec |
| **Token verification speed** | ~245k ops/sec | ~36k ops/sec | ~28k ops/sec |
| **Native async API** | ✅ (Tokio-backed) | ❌ | ❌ |
| **Access / refresh token model** | ✅ built-in | Manual | Manual |
| **Token rotation** | ✅ one call | Manual | Manual |
| **Key generation (HMAC + RSA)** | ✅ built-in | External tools | External tools |
| **Expiry tracking** (`remaining_time`) | ✅ | Manual | Manual |
| **HMAC (HS256/384/512)** | ✅ | ✅ | ✅ |
| **RSA (RS256/384/512)** | ✅ | ✅ | ✅ |
| **ECDSA (ES256/384)** | ✅ | ✅ | ✅ |
| **RSA-PSS (PS256/384/512)** | ✅ | ✅ | ❌ |
| **EdDSA (Ed25519)** | ✅ | ✅ | ❌ |
| **Python support** | 3.8+ (one ABI3 wheel), PyPy | 3.9+ | 3.6+ |

Performance figures are from our [HS256 benchmark](benchmarks.md).

---

## When to Choose CipherToken

- **High-traffic APIs** — authentication services, API gateways, and microservices where token operations are on the hot path.
- **Async frameworks** — FastAPI, Sanic, AioHTTP. CipherToken is the only option with a native async API; see the [async guide](advanced/async.md).
- **Full lifecycle out of the box** — access/refresh pairs, rotation, and expiry tracking without writing boilerplate.
- **Key management included** — generate HMAC secrets and RSA key pairs without extra dependencies.

## When PyJWT or python-jose May Fit Better

- You need a pure-Python dependency tree with no compiled extensions (e.g., restricted environments without prebuilt wheels for your platform — though CipherToken ships wheels for [most platforms](getting-started/installation.md)).
- You depend on JOSE features beyond JWT signing (e.g., JWE encryption in python-jose).

---

## Migrating from PyJWT

The core operations map directly:

=== "PyJWT"

    ```python
    import jwt
    import time
    import uuid

    SECRET = "your-strong-secret-key"

    # Create
    token = jwt.encode(
        {
            "user_id": 42,
            "exp": int(time.time()) + 600,
            "jti": str(uuid.uuid4()),
        },
        SECRET,
        algorithm="HS256",
    )

    # Verify + decode
    try:
        claims = jwt.decode(token, SECRET, algorithms=["HS256"])
        valid = True
    except jwt.InvalidTokenError:
        valid = False
    ```

=== "CipherToken"

    ```python
    from ciphertoken import CipherToken
    from ciphertoken.algorithms import HS256
    from ciphertoken.time import minutes

    ct = CipherToken(
        secret="your-strong-secret-key",
        algorithm=HS256,
        access_ttl=minutes(10),
    )

    # Create — exp and jti are added automatically
    token = ct.access(payload={"user_id": 42})

    # Verify + decode
    valid = ct.verify(token)
    claims = ct.decode(token)
    ```

Key differences to be aware of:

1. **Expiry is managed for you.** Configure `access_ttl` / `refresh_ttl` once instead of computing `exp` per token.
2. **`jti` is automatic.** Every token gets a UUID v4 identifier.
3. **Claims come back flattened.** `decode()` returns your payload keys at the top level alongside `exp`, `ttl`, `token_type`, and `jti`.
4. **`verify()` never raises.** It returns `True`/`False`; use `decode()` when you want a `ValueError` on invalid tokens.

---

## Next Steps

- [Install CipherToken](getting-started/installation.md)
- [Quick Start](getting-started/quick-start.md)
- [Benchmarks](benchmarks.md)
- [FAQ](faq.md)
