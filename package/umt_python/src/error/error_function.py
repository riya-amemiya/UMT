from typing import TypeVar

from .safe_execute import Error

# Unconstrained: errorFunction wraps any thrown value, including non-Exceptions.
E = TypeVar("E")


def error_function(error: E) -> Error[E]:
    """
    Creates an error result.

    Args:
        error: The error object

    Returns:
        An Error containing the error

    Example:
        >>> result = error_function(ValueError("test"))
        >>> result.type
        'error'
        >>> isinstance(result.error, ValueError)
        True
    """
    return Error(type="error", error=error)
