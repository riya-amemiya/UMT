from collections.abc import Callable
from dataclasses import dataclass
from typing import Generic, TypeVar

V = TypeVar("V")
E = TypeVar("E", bound=Exception)


@dataclass
class Success(Generic[V]):
    """Represents a successful execution result containing a value."""

    type: str = "success"
    value: V = None  # type: ignore
    error: None = None


@dataclass
class Error(Generic[E]):
    """Represents an error result containing an error object."""

    type: str = "error"
    error: E = None  # type: ignore
    value: None = None


Result = Success[V] | Error[E]


def safe_execute(callback: Callable[[], V]) -> Result[V, Exception]:
    """
    Safely executes a callback function and returns a Result type.
    Catches any errors and wraps them in a Result type.

    Args:
        callback: The function to execute safely

    Returns:
        A Result containing either the successful value or the caught error

    Example:
        >>> result = safe_execute(lambda: 1 / 0)
        >>> result.type
        'error'
        >>> result = safe_execute(lambda: 1 + 1)
        >>> result.value
        2
    """
    try:
        value = callback()
        return Success(type="success", value=value)
    except Exception as error:
        return Error(type="error", error=error)
