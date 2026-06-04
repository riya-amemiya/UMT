from typing import TypeVar

from .safe_execute import Success

V = TypeVar("V")


def success_function(value: V) -> Success[V]:
    """
    Creates a success result.

    Args:
        value: The success value

    Returns:
        A Success containing the value

    Example:
        >>> result = success_function(42)
        >>> result.type
        'success'
        >>> result.value
        42
    """
    return Success(type="success", value=value)
