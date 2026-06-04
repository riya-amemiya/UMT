from typing import Any

from .object_is_plain import object_is_plain


def flatten_object(
    object_: dict[str, Any],
    separator: str = ".",
) -> dict[str, Any]:
    """
    Recursively flattens a nested plain object into a single-level object
    keyed by joined paths. Arrays (lists) and non-plain objects are kept as-is.

    Empty plain objects are treated as leaf values rather than being removed.

    Args:
        object_ (dict[str, Any]): Source nested object.
        separator (str): Separator used to join path segments. Defaults to ".".

    Returns:
        dict[str, Any]: A flat object keyed by joined paths.

    Example:
        >>> flatten_object({"a": {"b": {"c": 1}}, "d": 2})
        {'a.b.c': 1, 'd': 2}
        >>> flatten_object({"a": {"b": 1}}, "/")
        {'a/b': 1}
        >>> flatten_object({"a": {}})
        {'a': {}}
    """
    result: dict[str, Any] = {}

    def walk(current: dict[str, Any], prefix: str) -> None:
        for key, value in current.items():
            next_key = f"{prefix}{separator}{key}" if prefix else key
            if object_is_plain(value) and len(value) > 0:
                walk(value, next_key)
            else:
                result[next_key] = value

    walk(object_, "")
    return result
