from typing import TypeVar

T = TypeVar("T")


def unwrap(value: T | None, message: str) -> T:
    """
    Unwraps a value that may be None, raising a ValueError if the value is absent.

    Args:
        value: The value to unwrap (may be None)
        message: The error message to raise if the value is absent

    Returns:
        The unwrapped value of type T

    Raises:
        ValueError: If the value is None
    """
    if value is not None:
        return value
    raise ValueError(message)
