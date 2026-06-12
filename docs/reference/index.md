# API Reference

Complete reference for all **CipherToken** modules, classes, and methods.

**CipherToken** is currently a **JWT-focused** token engine providing key generation, token lifecycle management, async support, and expiry tracking — all powered by **Rust**. Part of **[Cipher-Unit](https://cipherunit.xyz/)**.

```text
ciphertoken/
├── CipherToken              # Main token engine
├── secret                   # Key generation (HMAC, RSA)
├── time                     # Timestamp and TTL helpers
├── utils                    # Shared constants
├── algorithms               # Supported signing algorithms
└── jwt                      # High-level JWT convenience functions
```

| Module | Description | API Lookup |
|--------|-------------|------------|
| [`CipherToken`](ciphertoken.md) | Main class — sync and async methods | [⇗](api-lookup.md) |
| [`secret`](secret.md) | HMAC secret and RSA key pair generation | [⇗](api-lookup.md) |
| [`time`](time.md) | UNIX timestamp and TTL utilities | [⇗](api-lookup.md) |
| [`utils`](utils.md) | Shared constants | [⇗](api-lookup.md) |
| [`algorithms`](algorithms.md) | Algorithm constants | [⇗](api-lookup.md) |
| [`jwt`](jwt.md) | High-level JWT helpers (sync + async) | [⇗](api-lookup.md) |

> **New here?** Start with the [Quick Start](../getting-started/quick-start.md).
