from typing import Any

from .object_set import object_set


def unflatten_object(
    flat: dict[str, Any],
    separator: str = ".",
) -> dict[str, Any]:
    """
    Reconstructs a nested object from a flat path-keyed object.

    Each key is split on the separator into path segments and assigned with
    `object_set`, merging multiple paths into shared parents.

    Prototype pollution warning: internally uses `object_set`, which does not
    filter out prototype-polluting keys. Sanitize user-controlled keys.

    Args:
        flat (dict[str, Any]): Flat input keyed by joined paths.
        separator (str): Separator used in path keys. Defaults to ".".

    Returns:
        dict[str, Any]: A nested object.

    Example:
        >>> unflatten_object({"a.b.c": 1, "d": 2})
        {'a': {'b': {'c': 1}}, 'd': 2}
        >>> unflatten_object({"a.b": 1, "a.c": 2})
        {'a': {'b': 1, 'c': 2}}
        >>> unflatten_object({"a/b": 1}, "/")
        {'a': {'b': 1}}
    """
    result: dict[str, Any] = {}
    for key, value in flat.items():
        object_set(result, key.split(separator), value)
    return result
