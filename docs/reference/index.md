---
title: API Reference
description: Complete API reference for CipherToken modules — CipherToken, secret, time, utils, algorithms, and jwt. Built with Rust for Python.
keywords: ciphertoken api reference, python jwt api, jwt documentation, token api
image: https://cipherunits.github.io/CipherToken/logo.png
---

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

<div class="grid cards" markdown>

-   :fontawesome-solid-key: [CipherToken](ciphertoken.md)

-   :fontawesome-solid-lock: [secret](secret.md)

-   :fontawesome-solid-clock: [time](time.md)

-   :fontawesome-solid-cog: [utils](utils.md)

-   :fontawesome-solid-shield-halved: [algorithms](algorithms.md)

-   :fontawesome-solid-file-code: [jwt](jwt.md)

</div>

Looking for a specific method or constant? Use the [API Lookup index](api-lookup.md) — a single searchable page listing every symbol with its signature.

> **New here?** Start with the [Quick Start](../getting-started/quick-start.md).
