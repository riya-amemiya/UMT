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
        >>> is_admin = matches({"role": "admin"})
        >>> is_admin({"name": "Alice", "role": "admin"})
        True
        >>> is_admin({"name": "Bob", "role": "user"})
        False
    """

    def matcher(obj: dict[str, Any]) -> bool:
        for key, value in pattern.items():
            if key not in obj:
                return False
            obj_val = obj[key]
            if type(obj_val) is bool and type(value) is not bool:
                return False
            if type(obj_val) is not bool and type(value) is bool:
                return False
            if obj_val != value:
                return False
        return True

    return matcher
