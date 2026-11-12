from collections.abc import Callable
from typing import Any


def matches(pattern: dict[str, Any]) -> Callable[[dict[str, Any]], bool]:
    """
    Creates a predicate that checks whether an object matches
    all properties of the given pattern using strict equality

    Numbers and strings are compared by value (with bool kept distinct
    from numbers), while every other value is compared by identity,
    mirroring JavaScript's `===`.

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
            if isinstance(value, (int, float, str)):
                if (type(obj_val) is bool) != (type(value) is bool):
                    return False
                if not isinstance(obj_val, (int, float, str)) or obj_val != value:
                    return False
            elif obj_val is not value:
                return False
        return True

    return matcher
