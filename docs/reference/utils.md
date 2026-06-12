# API Reference — utils

The `utils` module contains shared constants used throughout the library.

## Constants

| Constant | Value | Description |
|----------|-------|-------------|
| `TOKEN_ACCESS` | `"access"` | String identifier for access tokens |
| `TOKEN_REFRESH` | `"refresh"` | String identifier for refresh tokens |
| `DEFAULT_SECRET_SIZE` | `32` | Default HMAC secret size in bytes |
| `MIN_SECRET_SIZE` | `16` | Minimum allowed HMAC secret size in bytes |

---

## Example

```python
from ciphertoken.utils import TOKEN_ACCESS, TOKEN_REFRESH

print(TOKEN_ACCESS)   # "access"
print(TOKEN_REFRESH)  # "refresh"
```
