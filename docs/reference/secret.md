---
title: secret Module — Key Generation API
description: Generate HMAC secrets and RSA key pairs with CipherToken's secret module. Secure random keys in PKCS#8 PEM format, sync and async.
keywords: generate hmac secret python, generate rsa key pair python, jwt secret key, pkcs8 pem python
image: https://cipherunits.github.io/CipherToken/logo.png
---

# API Reference — secret

The `secret` module provides utilities for cryptographic key generation.

## Functions

### `secret_key() -> str`

Generate a random HMAC secret key. Uses the default size (32 bytes).

```python
from ciphertoken.secret import secret_key

key = secret_key()
print(key)  # base64-encoded 32-byte random string
```

---

### `secret_key_with_size(size: int) -> str`

Generate a random HMAC secret key with a custom size.

```python
from ciphertoken.secret import secret_key_with_size

key = secret_key_with_size(64)
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `size` | `int` | Key size in bytes (minimum 16) |

**Raises:** `ValueError` if `size` is less than 16.

!!! warning "Minimum Size"
    The minimum allowed size for any HMAC secret is 16 bytes. Smaller values raise a `ValueError`.

---

### `generate_hmac_secret(size: int) -> str`

Explicitly generate an HMAC secret key. This is equivalent to `secret_key_with_size`.

```python
from ciphertoken.secret import generate_hmac_secret

secret = generate_hmac_secret(32)
```

---

### `generate_hmac_secret_async(size: int) -> str`

Asynchronous version of HMAC secret generation.

```python
import asyncio
from ciphertoken.secret import generate_hmac_secret_async

async def main():
    secret = await generate_hmac_secret_async(32)

asyncio.run(main())
```

---

### `generate_rsa_keypair(bits: Optional[int] = 2048) -> Tuple[str, str]`

Generate an RSA public/private key pair in PKCS#8 PEM format.

```python
from ciphertoken.secret import generate_rsa_keypair

private_key, public_key = generate_rsa_keypair(bits=2048)
```

**Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `bits` | `Optional[int]` | `2048` | Key size in bits. Minimum 2048 |

**Raises:** `ValueError` if `bits` is less than 2048.

**Returns:**

| Position | Type | Description |
|----------|------|-------------|
| `0` | `str` | RSA private key in PKCS#8 PEM format |
| `1` | `str` | RSA public key in PKCS#8 PEM format |

!!! tip "Recommended Key Sizes"
    Use **2048-bit** keys for most use cases and **3072-bit** or **4096-bit** keys for maximum security requirements. Keys smaller than 2048 bits are rejected. Note that key generation time grows steeply with size.
