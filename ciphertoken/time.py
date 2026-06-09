"""Python wrapper for Rust time submodule."""

from .ciphertoken import time as _time

now = _time.now
seconds = _time.seconds
minutes = _time.minutes
hours = _time.hours
days = _time.days
weeks = _time.weeks

__all__ = ["now", "seconds", "minutes", "hours", "days", "weeks"]
