# API Reference — time

The `time` module provides utility functions for UNIX timestamp management and TTL calculations.

## Functions

| Function | Signature | Description |
|----------|-----------|-------------|
| `now()` | `-> int` | Returns the current UNIX timestamp |
| `seconds(n)` | `(int) -> int` | Returns `n` unchanged (identity function) |
| `minutes(n)` | `(int) -> int` | Converts minutes to seconds (`n * 60`) |
| `hours(n)` | `(int) -> int` | Converts hours to seconds (`n * 3600`) |
| `days(n)` | `(int) -> int` | Converts days to seconds (`n * 86400`) |
| `weeks(n)` | `(int) -> int` | Converts weeks to seconds (`n * 604800`) |

**All functions accept a single integer argument and return an integer.**

---

## Example

```python
from ciphertoken.time import now, minutes, hours, days

print(now())       # e.g., 1716000000
print(minutes(5))  # 300
print(hours(2))    # 7200
print(days(7))     # 604800
```

---

## Usage with CipherToken

```python
from ciphertoken.time import minutes, days

token = CipherToken(
    secret="my-secret",
    algorithm=HS256,
    access_ttl=minutes(15),
    refresh_ttl=days(7),
)
```

These helpers make TTL values self-documenting and easy to adjust.
