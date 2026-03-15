from collections.abc import Callable
from typing import TypeVar

T = TypeVar("T")


def map_keys(object_: dict[str, T], function_: Callable[[T, str], str]) -> dict[str, T]:
    """
    Creates an object with the same values as the given object, but
    with keys transformed by the provided function.

    Args:
        object_ (dict[str, T]): The source object.
        function_ (Callable[[T, str], str]): The function invoked per key. Receives (value, key).

    Returns:
        dict[str, T]: A new object with transformed keys.

    Example:
        >>> map_keys({"a": 1, "b": 2}, lambda _value, key: key.upper())
        {'A': 1, 'B': 2}
    """
    result: dict[str, T] = {}
    for key, value in object_.items():
        result[function_(value, key)] = value
    return result
