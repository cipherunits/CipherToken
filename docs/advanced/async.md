# Advanced Guide — Async

**CipherToken** provides full asynchronous support via `pyo3-asyncio` and the `tokio` runtime. All token operations have async equivalents.

---

## When to Use Async

Use async methods when your application already runs inside an async framework such as **FastAPI**, **Sanic**, or **AioHTTP**. The async methods provide the same functionality as sync methods without blocking the event loop.

---

## Async API Surface

| Async Method | Sync Equivalent |
|--------------|-----------------|
| `access_async(payload) -> str` | `access(payload) -> str` |
| `refresh_async(payload) -> str` | `refresh(payload) -> str` |
| `decode_async(token) -> dict` | `decode(token) -> dict` |
| `verify_async(token) -> bool` | `verify(token) -> bool` |
| `rotation_async(refresh_token, payload) -> Tuple[str, str]` | `rotation(refresh_token, payload) -> Tuple[str, str]` |

---

## FastAPI Example

```python
from fastapi import FastAPI, Depends, HTTPException
from ciphertoken import CipherToken
from ciphertoken.algorithms import HS256
from ciphertoken.time import minutes, days

app = FastAPI()

# Shared token instance (initialize once at startup)
cipher = CipherToken(
    secret="your-strong-secret-key",
    algorithm=HS256,
    access_ttl=minutes(10),
    refresh_ttl=days(7),
)

@app.post("/auth/login")
async def login(username: str, password: str):
    # Validate credentials (pseudo-code)
    user_id = await authenticate_user(username, password)
    if not user_id:
        raise HTTPException(status_code=401, detail="Invalid credentials")

    access_token = await cipher.access_async(payload={"sub": username, "user_id": user_id})
    refresh_token = await cipher.refresh_async(payload={"sub": username, "user_id": user_id})

    return {"access_token": access_token, "refresh_token": refresh_token}


@app.post("/auth/refresh")
async def refresh_token_endpoint(refresh_token: str):
    is_valid = await cipher.verify_async(refresh_token)
    if not is_valid:
        raise HTTPException(status_code=401, detail="Invalid refresh token")

    payload = await cipher.decode_async(refresh_token)
    new_access, new_refresh = await cipher.rotation_async(
        refresh_token,
        payload={"sub": payload["payload"]["sub"], "user_id": payload["payload"]["user_id"]},
    )

    return {"access_token": new_access, "refresh_token": new_refresh}
```

---

## Direct `asyncio` Usage

```python
import asyncio
from ciphertoken import CipherToken
from ciphertoken.algorithms import HS256
from ciphertoken.time import minutes, days

cipher = CipherToken(
    secret="my-secret",
    algorithm=HS256,
    access_ttl=minutes(10),
    refresh_ttl=days(7),
)

async def main():
    access_token = await cipher.access_async(payload={"user_id": 1})
    refresh_token = await cipher.refresh_async(payload={"user_id": 1})

    claims = await cipher.decode_async(access_token)
    print(claims)

    new_access, new_refresh = await cipher.rotation_async(refresh_token)

    print(f"Valid: {await cipher.verify_async(access_token)}")

asyncio.run(main())
```

---

## Key Pair Generation (Async)

```python
import asyncio
from ciphertoken.secret import generate_rsa_keypair, generate_hmac_secret_async
from ciphertoken import CipherToken
from ciphertoken.algorithms import RS256
from ciphertoken.time import minutes

async def setup():
    private_key, public_key = generate_rsa_keypair(bits=4096)
    secret = await generate_hmac_secret_async(64)

    token_rsa = CipherToken(
        secret=private_key,
        algorithm=RS256,
        access_ttl=minutes(15),
        refresh_ttl=minutes(60),
    )

    token_hmac = CipherToken(
        secret=secret,
        algorithm=HS256,
        access_ttl=minutes(15),
        refresh_ttl=minutes(60),
    )

asyncio.run(setup())
```

---

## Async with `jwt` Module

```python
import asyncio
from ciphertoken.jwt import access_async, refresh_async, rotation_async

async def main():
    token = CipherToken(
        secret="secret",
        algorithm=HS256,
        access_ttl=minutes(10),
        refresh_ttl=days(7),
    )

    access_token = await access_async(token, payload={"sub": "user"})
    refresh_token = await refresh_async(token, payload={"sub": "user"})
    new_access, new_refresh = await rotation_async(token, refresh_token)

asyncio.run(main())
```

---

## Performance Notes

- CipherToken async methods use **`tokio`** underneath — they scale well across concurrent requests.
- There is **no performance penalty** for calling sync methods from sync contexts as long as you do not await them.
- See the [security guide](security.md) for guidance on secret handling in async environments.
