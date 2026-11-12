from collections.abc import Mapping

from .path_segments import path_segments


def _is_list_index(container: list[object], key: str) -> bool:
    return (
        key.isascii()
        and key.isdigit()
        and str(int(key)) == key
        and int(key) < len(container)
    )


def _unset_dict(value: Mapping[str, object], segments: list[str]) -> dict[str, object]:
    copy = dict(value)
    if len(segments) == 0 or segments[0] not in copy:
        return copy
    key = segments[0]
    if len(segments) > 1:
        copy[key] = _unset_segments(copy[key], segments[1:])
    else:
        del copy[key]
    return copy


def _unset_list(value: list[object], segments: list[str]) -> list[object]:
    copy = list(value)
    if not _is_list_index(copy, segments[0]):
        return copy
    index = int(segments[0])
    if len(segments) > 1:
        copy[index] = _unset_segments(copy[index], segments[1:])
    else:
        del copy[index]
    return copy


def _unset_segments(value: object, segments: list[str]) -> object:
    if isinstance(value, dict):
        return _unset_dict(value, segments)
    if isinstance(value, list):
        return _unset_list(value, segments)
    return value


def object_unset(
    object_: Mapping[str, object], path: str | list[str]
) -> dict[str, object]:
    """
    Removes the property at a deeply nested path without mutating the input.
    Uses the same path parsing as `object_get`, `object_set` and `object_has`.

    Dicts and lists along the path are shallow-copied, while untouched branches
    keep their references. Removing a list index removes the element and shifts
    the following elements. Lists are traversed by canonical decimal index
    segments (for example "0"). Any other value on the path, or a missing
    segment, stops the traversal and leaves the rest unchanged.

    Prototype pollution warning: this function does not filter out
    prototype-polluting keys ("__proto__", "constructor", "prototype").
    Sanitize user-controlled paths before calling.

    Args:
        object_ (Mapping[str, object]): Source object (not mutated).
        path (str | list[str]): Dot-separated string or list of path segments.

    Returns:
        dict[str, object]: A new object without the property at the path.

    Example:
        >>> object_unset({"a": {"b": 1, "c": 2}}, "a.b")
        {'a': {'c': 2}}
        >>> object_unset({"a": [1, 2, 3]}, ["a", "1"])
        {'a': [1, 3]}
        >>> object_unset({"a": 1}, "x.y")
        {'a': 1}
    """
    return _unset_dict(object_, path_segments(path))
