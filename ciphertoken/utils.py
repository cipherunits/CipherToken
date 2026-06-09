"""Python wrapper for Rust utils submodule."""

from .ciphertoken import utils as _utils

DEFAULT_SECRET_SIZE = _utils.DEFAULT_SECRET_SIZE
MIN_SECRET_SIZE = _utils.MIN_SECRET_SIZE
TOKEN_ACCESS = _utils.TOKEN_ACCESS
TOKEN_REFRESH = _utils.TOKEN_REFRESH

__all__ = ["DEFAULT_SECRET_SIZE", "MIN_SECRET_SIZE", "TOKEN_ACCESS", "TOKEN_REFRESH"]
