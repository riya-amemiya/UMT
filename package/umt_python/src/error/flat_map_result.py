from collections.abc import Callable
from typing import TypeVar

from .safe_execute import Result, Success

V = TypeVar("V")
E = TypeVar("E", bound=Exception)
U = TypeVar("U")
F = TypeVar("F", bound=Exception)


def flat_map_result(result: Result[V, E], f: Callable[[V], Result[U, F]]) -> Result[U, E] | Result[U, F]:
    """
    Transforms the value inside a successful Result using a function
    that itself returns a Result. If the original Result is an error,
    it is returned unchanged.

    Args:
        result: The Result to transform
        f: The function to apply to the success value, which returns a new Result

    Returns:
        The Result returned by the mapping function, or the original error

    Example:
        >>> from .safe_execute import Success, Error
        >>> success = Success(value=5)
        >>> flat_map_result(success, lambda n: Success(value=n * 2) if n > 0 else Error(error=ValueError("negative"))).value
        10
    """
    if isinstance(result, Success):
        return f(result.value)
    return result
