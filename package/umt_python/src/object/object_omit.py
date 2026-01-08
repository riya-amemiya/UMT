from typing import Dict, TypeVar

T = TypeVar("T")


def object_omit(obj: Dict[str, T], *keys: str) -> Dict[str, T]:
    """
    Creates an object without the specified keys.

    Args:
        obj: The source object
        *keys: The keys to omit

    Returns:
        A new object without the specified keys

    Example:
        >>> object_omit({"a": 1, "b": 2, "c": 3}, "b", "c")
        {'a': 1}
    """
    result = obj.copy()
    for key in keys:
        result.pop(key, None)
    return result
