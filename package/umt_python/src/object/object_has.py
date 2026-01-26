from typing import Any


def object_has(obj: dict[str, Any], path: str | list[str]) -> bool:
    """
    Determines if an object has a specified path.

    Args:
        obj: Object to check
        path: Path to check (string with dots or list of strings)

    Returns:
        True if path exists, False otherwise

    Example:
        >>> object_has({"a": {"b": 1}}, "a.b")
        True
        >>> object_has({"a": {"b": 1}}, ["a", "b"])
        True
        >>> object_has({"a": {"b": 1}}, "a.c")
        False
    """
    local_path = path.split(".") if isinstance(path, str) else path
    current = obj.copy()

    for key in local_path:
        if current is None or key not in current:
            return False
        current = current[key]

    return True
