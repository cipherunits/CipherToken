# CipherToken

**CipherToken** is a next-generation token engine for developers who demand speed, security, and reliability. Unlike conventional JWT libraries, CipherToken delivers a complete token lifecycle — from creation and decoding to rotation and expiry tracking — all backed by the raw performance of **Rust**.

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

---

## Quick Install

```bash
pip install ciphertoken
```

---

## Quick Example

```python
from ciphertoken import CipherToken
from ciphertoken.algorithms import HS256
from ciphertoken.time import minutes, days
from ciphertoken.jwt import access, refresh, rotation

token = CipherToken(
    secret="your-strong-secret-key",
    algorithm=HS256,
    access_ttl=minutes(10),
    refresh_ttl=days(7),
)

access_token = access(token, payload={"user_id": 42})
refresh_token = refresh(token, payload={"user_id": 42})
new_access, new_refresh = rotation(token, refresh_token)

print(token.verify(access_token))  # True
```

---

## Get Started

<div class="grid cards" markdown>

-   :fontawesome-solid-download: [Installation](getting-started/installation.md) — pip, poetry, pdm, and more

-   :fontawesome-solid-bolt: [Quick Start](getting-started/quick-start.md) — Your first tokens in under 2 minutes

-   :fontawesome-solid-book: [API Reference](reference/index.md) — Complete module documentation

-   :fontawesome-solid-gear: [Advanced Guide](advanced/index.md) — Production best practices

</div>