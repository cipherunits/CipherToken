# Installation

Choose your preferred method to install **CipherToken**.

---

## System Requirements

- Python **3.8** or higher
- A C compiler (GCC, Clang, or MSVC) for builds from source

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



---

## Building from Source

For the latest unreleased code:

```bash
git clone https://github.com/cipherunits/CipherToken.git
cd CipherToken
pip install maturin
maturin develop --release
```

!!! tip
    `maturin develop --release` compiles the Rust extension and installs it into your active Python environment. Use `--release` for production-grade performance.

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

**Build errors on Linux**

```bash
# Debian / Ubuntu
sudo apt install build-essential python3-dev

# Fedora
sudo dnf install gcc python3-devel
```

**Build errors on Windows**

Install **Microsoft C++ Build Tools** or Visual Studio with the "Desktop development with C++" workload.

---

➡️ [Quick Start](quick-start.md) — Generate your first tokens.
