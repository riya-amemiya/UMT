from collections.abc import Callable
from typing import TypeVar

from .safe_execute import Result, Success

V = TypeVar("V")
E = TypeVar("E", bound=Exception)
U = TypeVar("U")


def map_result(result: Result[V, E], f: Callable[[V], U]) -> Result[U, E]:
    """
    Transforms the value inside a successful Result using the provided mapping function.
    If the Result is an error, it is returned unchanged.

    Args:
        result: The Result to transform
        f: The function to apply to the success value

    Returns:
        A new Result with the transformed value, or the original error

    Example:
        >>> success = Success(value=5)
        >>> map_result(success, lambda n: n * 2).value
        10
        >>> error = Error(error=ValueError("fail"))
        >>> map_result(error, lambda n: n * 2).type
        'error'
    """
    if isinstance(result, Success):
        return Success(value=f(result.value))
    return result
