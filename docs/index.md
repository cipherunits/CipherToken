---
title: Fast Python JWT Library Built with Rust
description: CipherToken is a high-performance Python JWT library built with Rust. Up to 6x faster than PyJWT, with async support, key generation, and token rotation.
keywords: python jwt library, fast jwt python, rust jwt, jwt token python, pyjwt alternative, jwt authentication python, ciphertoken
image: https://cipherunits.github.io/CipherToken/logo.png
---

# CipherToken — Fast Python JWT Library Built with Rust

**CipherToken** is a next-generation token engine for developers who demand speed, security, and reliability. Unlike conventional JWT libraries, CipherToken delivers a complete token lifecycle — from creation and decoding to rotation and expiry tracking — all backed by the raw performance of **Rust**.

```bash
pip install ciphertoken
```

<div class="grid cards" markdown>

-   :fontawesome-solid-gauge-high: **Up to 6x faster** than PyJWT and python-jose — [see benchmarks](benchmarks.md)

-   :fontawesome-solid-bolt: **Fully async** — every operation has an async twin, backed by Tokio

-   :fontawesome-solid-key: **Built-in key generation** — HMAC secrets and RSA key pairs from Python

-   :fontawesome-solid-arrows-rotate: **Complete lifecycle** — create, verify, decode, inspect, rotate, track expiry

</div>

---

## Quick Example

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

access_token = token.access(payload={"user_id": 42})
refresh_token = token.refresh(payload={"user_id": 42})
new_access, new_refresh = token.rotation(refresh_token)

print(token.verify(access_token))  # True
```

Ready for more? Follow the [Quick Start guide](getting-started/quick-start.md).

---

## Why CipherToken?

| | Conventional JWT Libraries | CipherToken |
|---|---|---|
| **Language** | Pure Python | Rust + PyO3 |
| **Performance** | Interpreted overhead | Near-native speed |
| **Async** | Often limited or absent | Fully async (Tokio) |
| **Token lifecycle** | Generate / verify | Create · Decode · Verify · Rotate · Inspect |
| **Key management** | Manual | Built-in HMAC + RSA key generation |
| **Expiry tracking** | Manual | Built-in (`remaining_time`) |

See the full [comparison with PyJWT and python-jose](comparison.md).

---

## Performance at a Glance

CipherToken outperformed every Python JWT library we tested, in every category:

| Library | Token Creation (ops/sec) | Token Verification (ops/sec) |
|----------|----------:|----------:|
| **CipherToken** | **587,156** | **244,500** |
| PyJWT | 101,591 | 35,928 |
| python-jose | 103,861 | 27,638 |

Full methodology, environment, and analysis on the [Benchmarks page](benchmarks.md).

---

## Broad Platform Support

One `pip install` works everywhere — prebuilt wheels ship for:

- **Linux** — x86_64, ARM64, ARMv7, s390x, ppc64le (glibc and musl/Alpine)
- **Windows** — x64, x86, ARM64
- **macOS** — Intel and Apple Silicon
- **Python** — CPython 3.8+ (one ABI3 wheel covers them all) and PyPy 3.9/3.10

---

## Get Started

<div class="grid cards" markdown>

-   :fontawesome-solid-download: [Installation](getting-started/installation.md) — pip, uv, poetry, pdm, and more

-   :fontawesome-solid-bolt: [Quick Start](getting-started/quick-start.md) — Your first tokens in under 2 minutes

-   :fontawesome-solid-book: [API Reference](reference/index.md) — Complete module documentation

-   :fontawesome-solid-gear: [Advanced Guide](advanced/index.md) — Async usage and security best practices

-   :fontawesome-solid-circle-question: [FAQ](faq.md) — Frequently asked questions

-   :fontawesome-solid-scale-balanced: [Comparison](comparison.md) — CipherToken vs PyJWT vs python-jose

</div>
