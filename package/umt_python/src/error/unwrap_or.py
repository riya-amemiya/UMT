from typing import TypeVar

from .safe_execute import Result, Success

V = TypeVar("V")
E = TypeVar("E")
D = TypeVar("D")


def unwrap_or(result: Result[V, E], default_value: D) -> V | D:
    """
    Extracts the value from a successful Result, or returns the provided
    default value if the Result is an error.

    Args:
        result: The Result to unwrap
        default_value: The value returned when the Result is an error

    Returns:
        The success value, or the default value for an error Result

    Example:
        >>> unwrap_or(Success(value=5), 0)
        5
        >>> from .safe_execute import Error
        >>> unwrap_or(Error(error=ValueError("fail")), 0)
        0
    """
    if isinstance(result, Success):
        return result.value
    return default_value
