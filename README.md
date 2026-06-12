<img src="https://github.com/cipherunits/CipherToken/blob/main/logo.png" width=100>

# CipherToken

**CipherToken** is a **next-generation** token engine built for developers who demand speed, security, and reliability. Currently focused on **JWT**, it provides a complete token lifecycle — from key generation and token minting to verification, rotation, and expiry tracking — all backed by the raw performance of **Rust**.

- Python 3.8+
- HMAC, RSA, ECDSA, RSA-PSS, EdDSA
- Sync and Async APIs via **Tokio**

Built by **[Cipher-Unit](https://cipherunit.xyz/)**.

---

## Documentation

📖 **Official Documentation:** [https://cipherunits.github.io/CipherToken/](https://cipherunits.github.io/CipherToken/)

---

## Installation

```bash
pip install ciphertoken
```

---

## Quick Example

```python
from ciphertoken import CipherToken
from ciphertoken.algorithms import HS256
from ciphertoken.time import minutes, days
from ciphertoken.jwt import access, refresh, rotation

token = CipherToken(
    secret="your-strong-secret-key",
    algorithm=HS256,
    access_ttl=minutes(10),
    refresh_ttl=days(7),
)

access_token = access(token, payload={"user_id": 42})
refresh_token = refresh(token, payload={"user_id": 42})
new_access, new_refresh = rotation(token, refresh_token)

print(token.verify(access_token))  # True
print(token.decode(access_token))
```

---

## Contribute

Contributions are welcome! Whether it's bug fixes, new algorithms, documentation improvements, or feature suggestions — feel free to open an issue or pull request.

- **Repository:** [https://github.com/cipherunits/CipherToken](https://github.com/cipherunits/CipherToken)
- **Issues:** [https://github.com/cipherunits/CipherToken/issues](https://github.com/cipherunits/CipherToken/issues)

Please read the code of conduct and contribution guidelines before submitting. All contributions must follow the MIT license.

---

## License

**MIT** — see [LICENSE](LICENSE) for details.
