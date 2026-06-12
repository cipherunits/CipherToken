# Advanced Guide

Production topics for **CipherToken** — the next-generation token engine.

## Topics

- [Async Usage](async.md) — Full async API with Tokio under the hood
- [Security Best Practices](security.md) — Key management, rotation, algorithm selection

---

## Async First

Developed by **[Cipher-Unit](https://cipherunit.xyz/)**. Every sync method has an async twin, backed by **Tokio**. No secondary package, no compatibility layers — it is part of the core engine.

!!! tip "Production async"
    Use `CipherToken` async methods in FastAPI, Sanic, or AioHTTP for GIL-free token operations at scale.
