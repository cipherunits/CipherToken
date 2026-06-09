"""Python wrapper for Rust algorithms submodule."""

from .ciphertoken import algorithms as _algorithms

HS256 = _algorithms.HS256
HS384 = _algorithms.HS384
HS512 = _algorithms.HS512
RS256 = _algorithms.RS256
RS384 = _algorithms.RS384
RS512 = _algorithms.RS512
ES256 = _algorithms.ES256
ES384 = _algorithms.ES384
PS256 = _algorithms.PS256
PS384 = _algorithms.PS384
PS512 = _algorithms.PS512
EDDSA = _algorithms.EDDSA

__all__ = ["HS256", "HS384", "HS512", "RS256", "RS384", "RS512", "ES256", "ES384", "PS256", "PS384", "PS512", "EDDSA"]
