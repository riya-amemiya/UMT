from collections.abc import Callable
from typing import TypeVar

from .safe_execute import Error, Result

V = TypeVar("V")
E = TypeVar("E")
F = TypeVar("F")


def map_error(result: Result[V, E], f: Callable[[E], F]) -> Result[V, F]:
    """
    Transforms the error inside a failed Result using the provided mapping function.
    If the Result is a success, it is returned unchanged.

    Args:
        result: The Result to transform
        f: The function to apply to the error

    Returns:
        A new Result with the transformed error, or the original success

    Example:
        >>> map_error(Error(error="not found"), lambda message: f"wrapped: {message}").error
        'wrapped: not found'
        >>> from .safe_execute import Success
        >>> map_error(Success(value=5), lambda message: f"wrapped: {message}").value
        5
    """
    if isinstance(result, Error):
        return Error(type="error", error=f(result.error))
    return result
