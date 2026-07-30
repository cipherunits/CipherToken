---
title: Advanced Guide
description: Production topics for CipherToken — native async JWT operations and security best practices for token infrastructure.
keywords: ciphertoken advanced, async jwt, jwt security, production jwt python
image: https://cipherunits.github.io/CipherToken/logo.png
---

# Advanced Guide

Production topics for **CipherToken** — the next-generation token engine.

## Topics

<div class="grid cards" markdown>

-   :fontawesome-solid-bolt: [Async Usage](async.md)

-   :fontawesome-solid-shield: [Security Best Practices](security.md)

</div>

---

## Async First

Developed by **[Cipher-Unit](https://cipherunit.xyz/)**. Every sync method has an async twin, backed by **Tokio**. No secondary package, no compatibility layers — it is part of the core engine.

!!! tip "Production async"
    Use `CipherToken` async methods in FastAPI, Sanic, or AioHTTP for GIL-free token operations at scale.
