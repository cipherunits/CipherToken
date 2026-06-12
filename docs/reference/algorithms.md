# API Reference — algorithms

The `algorithms` module defines constants for all supported JWT signing algorithms.

## Supported Algorithms

### HMAC

| Constant | Description |
|----------|-------------|
| `HS256` | HMAC using SHA-256 |
| `HS384` | HMAC using SHA-384 |
| `HS512` | HMAC using SHA-512 |

### RSA

| Constant | Description |
|----------|-------------|
| `RS256` | RSASSA-PKCS1-v1_5 using SHA-256 |
| `RS384` | RSASSA-PKCS1-v1_5 using SHA-384 |
| `RS512` | RSASSA-PKCS1-v1_5 using SHA-512 |

### ECDSA

| Constant | Description |
|----------|-------------|
| `ES256` | ECDSA using P-256 and SHA-256 |
| `ES384` | ECDSA using P-384 and SHA-384 |

### RSA-PSS

| Constant | Description |
|----------|-------------|
| `PS256` | RSASSA-PSS using SHA-256 |
| `PS384` | RSASSA-PSS using SHA-384 |
| `PS512` | RSASSA-PSS using SHA-512 |

### Edwards Curve

| Constant | Description |
|----------|-------------|
| `EDDSA` | EdDSA using Ed25519 |

---

## Example

```python
from ciphertoken.algorithms import HS256, RS256, EDDSA

# HS256: symmetric (requires a shared secret key)
# RS256: asymmetric (requires RSA private/public key pair)
# EDDSA: asymmetric (requires Ed25519 key pair)
```

---

## Choosing an Algorithm

| Scenario | Recommended Algorithm | Key Type |
|----------|----------------------|----------|
| Simple services, single secret | `HS256` | Shared secret (HMAC) |
| Public key verification | `RS256`, `ES256`, `EDDSA` | Key pair (asymmetric) |
| Maximum security / compliance | `PS512`, `RS512` | Key pair (RSA 4096-bit) |

!!! tip
    **HMAC (`HS*`)** algorithms require a shared secret and are faster. **RSA/ECDSA/EdDSA** algorithms use a key pair and enable public key verification.
