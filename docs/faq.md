---
title: Frequently Asked Questions
description: Answers to common questions about CipherToken — Python version support, platforms, async usage, security, claims structure, and migration from PyJWT.
keywords: ciphertoken faq, python jwt questions, jwt library help, ciphertoken support
image: https://cipherunits.github.io/CipherToken/logo.png
---

# Frequently Asked Questions

Common questions about **CipherToken**. Can't find your answer? [Open an issue on GitHub](https://github.com/cipherunits/CipherToken/issues).

---

## General

### What is CipherToken?

CipherToken is a high-performance JWT library for Python, written in Rust with PyO3. It covers the complete token lifecycle: key generation, token creation, verification, decoding, rotation, and expiry tracking. See the [Getting Started guide](getting-started/index.md).

### Why is CipherToken faster than PyJWT?

The entire signing and verification pipeline runs in compiled Rust — Python only pays the cost of a single native call. In our [benchmarks](benchmarks.md), CipherToken was 5.7–8.9x faster than PyJWT and python-jose.

### Is CipherToken production-ready?

CipherToken builds on battle-tested Rust crates: `jsonwebtoken` for JWT operations, `rsa` for key generation, and `tokio` for async. Signing and verification are never hand-rolled.

---

## Compatibility

### Which Python versions are supported?

CPython **3.8 and newer** — a single ABI3 wheel covers all of them, including the latest releases. PyPy 3.9 and 3.10 wheels are also published.

### Which platforms have prebuilt wheels?

Linux (x86_64, ARM64, ARMv7, s390x, ppc64le — both glibc and musl/Alpine), Windows (x64, x86, ARM64), and macOS (Intel and Apple Silicon). If your platform isn't listed, `pip` builds from source automatically — see [Installation](getting-started/installation.md).

### Does CipherToken work with FastAPI, Sanic, and AioHTTP?

Yes. Every operation has a native async equivalent (`access_async`, `verify_async`, `rotation_async`, ...) that doesn't block the event loop. See the [async guide](advanced/async.md) for a complete FastAPI example.

---

## Usage

### What claims does every token contain?

Four standard claims are added automatically, and your payload keys are merged alongside them at the top level:

| Claim | Description |
|-------|-------------|
| `exp` | UNIX expiry timestamp |
| `ttl` | Time-to-live in seconds at creation |
| `token_type` | `"access"` or `"refresh"` |
| `jti` | Unique UUID v4 identifier |

```python
claims = ct.decode(token)
claims["user_id"]     # your payload key — top level, not nested
claims["token_type"]  # "access"
```

### What's the difference between `verify`, `decode`, and `inspect`?

- `verify(token)` — returns `True`/`False`, never raises. Use for quick validity checks.
- `decode(token)` — returns all claims, raises `ValueError` on invalid or expired tokens.
- `inspect(token)` — decodes **without** strict validation. Use for debugging expired tokens, never for authorization.

### How do I rotate refresh tokens?

One call returns a fresh access + refresh pair:

```python
new_access, new_refresh = ct.rotation(old_refresh_token, payload={"user_id": 42})
```

Passing an access token raises `ValueError` — only refresh tokens can be rotated.

### Which algorithm should I use?

`HS256` for internal services with a shared secret; `RS256`, `ES256`, or `EDDSA` when third parties need to verify tokens with a public key. Full guidance in [Algorithm Selection](advanced/security.md#algorithm-selection).

---

## Security

### How should I store my secret key?

Never hardcode it. Use environment variables or a secret manager, and generate it with the built-in helper:

```python
from ciphertoken.secret import secret_key_with_size
secret = secret_key_with_size(64)  # store this securely, once
```

See [Security Best Practices](advanced/security.md).

### Does CipherToken support the `none` algorithm?

No. The unsigned `none` algorithm is a well-known JWT attack vector and is not exposed.

### What are the recommended token lifetimes?

Short-lived access tokens (5–15 minutes) and longer refresh tokens (hours to days) matching your session policy. Details in [Token Expiry](advanced/security.md#token-expiry).

---

## Migration

### How do I migrate from PyJWT?

The core operations map one-to-one — see the [side-by-side migration guide](comparison.md#migrating-from-pyjwt).

### Does CipherToken support JWE (encrypted tokens)?

Not currently. CipherToken focuses on signed JWTs (JWS). If you need encryption, encrypt the payload before adding it to the token.
