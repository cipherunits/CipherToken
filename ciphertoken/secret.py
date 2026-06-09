"""Python wrapper for Rust secret submodule."""

from .ciphertoken import secret as _secret

secret_key = _secret.secret_key
secret_key_with_size = _secret.secret_key_with_size
generate_hmac_secret = _secret.generate_hmac_secret
generate_hmac_secret_async = _secret.generate_hmac_secret_async
generate_rsa_keypair = _secret.generate_rsa_keypair

__all__ = [
    "secret_key",
    "secret_key_with_size",
    "generate_hmac_secret",
    "generate_hmac_secret_async",
    "generate_rsa_keypair",
]
