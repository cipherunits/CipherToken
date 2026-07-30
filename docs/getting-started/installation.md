---
title: Installation
description: Install CipherToken, the high-performance Python JWT library built with Rust. Prebuilt wheels for Linux, Windows, macOS, ARM, Alpine, and PyPy.
keywords: install ciphertoken, pip install jwt, python jwt install, poetry, uv, pipx, alpine wheel, arm64 wheel
image: https://cipherunits.github.io/CipherToken/logo.png
---

# Installation

Choose your preferred method to install **CipherToken**.

---

## System Requirements

- Python **3.8** or higher (CPython) or PyPy 3.9/3.10
- Nothing else — prebuilt wheels ship for all major platforms
- The **Rust toolchain**, only if you build from source

---

## Install

=== "pip"
    ```bash
    pip install ciphertoken
    ```

    Installs the latest wheel from PyPI — ABI3 compatible, works on Python 3.8+.

=== "uv"
    ```bash
    uv pip install ciphertoken
    ```

    Fast installation using `uv`. Ideal for developers who already use `uv` as their package manager.

=== "poetry"
    ```bash
    poetry add ciphertoken
    ```

=== "pdm"
    ```bash
    pdm add ciphertoken
    ```

=== "pipx"
    ```bash
    pipx install ciphertoken
    ```
    Run CipherToken in an isolated environment without affecting your global Python installation.

---

## Supported Platforms

Prebuilt wheels are published to PyPI for every release:

| Platform | Architectures |
|----------|---------------|
| **Linux** (glibc, manylinux) | x86_64, x86, ARM64, ARMv7, s390x, ppc64le |
| **Linux** (musl / Alpine) | x86_64, x86, ARM64, ARMv7 |
| **Windows** | x64, x86, ARM64 |
| **macOS** | Intel (x86_64), Apple Silicon (ARM64) |
| **PyPy 3.9 / 3.10** | Linux, Windows, macOS |

!!! info "One wheel, every Python version"
    CipherToken wheels use the **stable ABI (abi3)** — a single wheel works on
    CPython 3.8 through the latest release, so new Python versions are
    supported on day one.

On any other platform, `pip` automatically falls back to building from source (see below).

---

## Building from Source

For unsupported platforms or the latest unreleased code:

```bash
git clone https://github.com/cipherunits/CipherToken.git
cd CipherToken
pip install maturin
maturin develop --release
```

!!! tip
    `maturin develop --release` compiles the Rust extension and installs it into your active Python environment. Use `--release` for production-grade performance.

Building from source requires the [Rust toolchain](https://rustup.rs/) (`rustc` and `cargo`).

---

## Verifying Installation

```python
>>> from ciphertoken import CipherToken
>>> from ciphertoken.algorithms import HS256
>>> from ciphertoken.time import minutes
>>> CipherToken(secret="test", algorithm=HS256, access_ttl=minutes(5), refresh_ttl=minutes(10))
<ciphertoken.ciphertoken.CipherToken object at 0x...>
```

---

## Troubleshooting

**No matching distribution found**

Make sure `pip` is up to date:

```bash
pip install --upgrade pip setuptools wheel
```

**Build errors on Linux (source builds only)**

```bash
# Install Rust first
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Debian / Ubuntu
sudo apt install build-essential python3-dev

# Fedora
sudo dnf install gcc python3-devel
```

**Build errors on Windows (source builds only)**

Install the [Rust toolchain](https://rustup.rs/) and **Microsoft C++ Build Tools** (or Visual Studio with the "Desktop development with C++" workload).

---

➡️ [Quick Start](quick-start.md) — Generate your first tokens.
