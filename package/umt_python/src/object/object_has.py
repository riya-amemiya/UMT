from typing import Any


def object_has(obj: dict[str, Any], path: str | list[str]) -> bool:
    """
    Determines if an object has a specified path.

    Lists are traversed by canonical decimal index segments (for example
    "0"), mirroring own index keys of JavaScript arrays. Stepping through
    any other non-dict value returns False.

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
        >>> object_has({"list": [10, 20]}, "list.0")
        True
    """
    local_path = path.split(".") if isinstance(path, str) else path
    current: Any = obj

    for key in local_path:
        if isinstance(current, dict):
            if key not in current:
                return False
            current = current[key]
        elif (
            isinstance(current, list)
            and key.isascii()
            and key.isdigit()
            and str(int(key)) == key
            and int(key) < len(current)
        ):
            current = current[int(key)]
        else:
            return False

    return True
