from collections.abc import Callable
from typing import Any


def matches(pattern: dict[str, Any]) -> Callable[[dict[str, Any]], bool]:
    """
    Creates a predicate that checks whether an object matches
    all properties of the given pattern using strict equality

    Args:
        pattern: The pattern to match against

    Returns:
        A predicate that tests objects

    Example:
        >>> is_admin = matches({ "role": "admin" })
        >>> is_admin({ "name": "Alice", "role": "admin" })
        True
        >>> is_admin({ "name": "Bob", "role": "user" })
        False
    """

    def predicate(obj: dict[str, Any]) -> bool:
        for key, expected_value in pattern.items():
            if key not in obj:
                return False
            actual_value = obj[key]
            if type(actual_value) is bool and type(expected_value) is not bool:
                return False
            if type(expected_value) is bool and type(actual_value) is not bool:
                return False
            if actual_value != expected_value:
                return False
        return True

    return predicate
