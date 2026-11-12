from collections.abc import Mapping

from .object_unset import object_unset


def object_omit_deep(obj: Mapping[str, object], *keys: str) -> dict[str, object]:
    """
    Creates a new object by deeply removing properties from the source object
    based on specified keys. This is the counterpart of `object_pick_deep`.

    Each key is removed with `object_unset`, so dicts and lists along a path are
    shallow-copied and the source object is not mutated. Keys that do not exist
    are ignored.

    Args:
        obj: The source object to remove properties from
        *keys: Property keys to remove. Can use dot notation for nested properties.

    Returns:
        A new object without the specified properties

    Example:
        >>> obj = {"a": {"b": {"c": 1, "d": 2}, "e": 3}, "f": 4}
        >>> object_omit_deep(obj, "a.b.c", "f")
        {'a': {'b': {'d': 2}, 'e': 3}}
    """
    result = object_unset(obj, [])
    for key in keys:
        result = object_unset(result, key)
    return result
