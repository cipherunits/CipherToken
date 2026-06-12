# Getting Started

Welcome to **CipherToken** — the next-generation token management library.

## What is CipherToken?

**CipherToken** is a **next-generation** token engine built for developers who demand speed, security, and reliability. Unlike conventional JWT libraries limited to simple generation and verification, CipherToken delivers a **complete token lifecycle** — from creation and decoding to rotation and expiry tracking — all backed by the raw performance of **Rust**.

Built on **PyO3** and **Rust**, CipherToken compiles down to native machine code, giving you near-zero overhead cryptography in a Pythonic interface. While focused on **JWT** today, it is architected as a modular token layer that will expand beyond JWT in future releases.

---


## Why Next-Generation?

| | Conventional JWT Libraries | CipherToken |
|---|---|---|
| **Language** | Pure Python | Rust + PyO3 |
| **Performance** | Interpreted overhead | Near-native speed |
| **Async** | Often limited or absent | Fully async (Tokio) |
| **Token lifecycle** | Generate / verify | Create · Decode · Verify · Rotate · Inspect |
| **Key management** | Manual | Built-in HMAC + RSA key generation |
| **Expiry tracking** | Manual | Built-in (`remaining_time`) |
| **Type safety** | Runtime | Compiled Rust core |

---

## Key Features

- **Built with Rust** — Compiled to native code via PyO3 for maximum throughput
- **High Performance** — Token operations in microseconds, GIL-free for async workloads
- **Complete Token Lifecycle** — Sign, verify, decode, inspect, rotate, and track expiry
- **Sync and Async** — Drop-in async equivalents for every operation
- **Multi-Algorithm** — HMAC, RSA, ECDSA, RSA-PSS, EdDSA out of the box
- **Built-in Key Generation** — HMAC secrets and RSA key pairs directly from Python
- **ABI3 Compatible** — One wheel for Python 3.8+

---

## What CipherToken is (and isn't)

> **CipherToken is built for JWT right now.** It provides a complete JWT lifecycle: key generation, token minting, validation, rotation, and expiry tracking.
> It is architected to expand beyond JWT in future releases, but today its scope is focused on JWT.

---

## Project

Developed and maintained by **[Cipher-Unit](https://cipherunit.xyz/)**.

<div class="grid cards" markdown>

- [Installation](installation.md) — pip, poetry, and other package managers
- [Quick Start](quick-start.md) — Your first tokens in under 2 minutes
- [API Reference](../reference/index.md) — Complete API documentation

</div>
