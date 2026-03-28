from collections.abc import Callable
from typing import TypeVar

from .safe_execute import Result, Success

V = TypeVar("V")
E = TypeVar("E", bound=Exception)
S = TypeVar("S")
F = TypeVar("F")


def match_result(
    result: Result[V, E],
    on_success: Callable[[V], S],
    on_error: Callable[[E], F],
) -> S | F:
    """
    Pattern-matches on a Result, applying the appropriate handler
    depending on whether the Result is a success or an error.

    Args:
        result: The Result to match on
        on_success: Called with the value if the Result is a success
        on_error: Called with the error if the Result is an error

    Returns:
        The return value of whichever handler is invoked

    Example:
        >>> from .safe_execute import Success
        >>> success = Success(value=42)
        >>> match_result(success, lambda v: f"Got {v}", lambda e: f"Failed: {e}")
        'Got 42'
    """
    if isinstance(result, Success):
        return on_success(result.value)
    return on_error(result.error)
