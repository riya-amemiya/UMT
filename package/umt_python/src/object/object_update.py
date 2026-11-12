from collections.abc import Callable
from typing import TypeVar

from .object_get import object_get
from .object_set import object_set
from .path_segments import path_segments

T = TypeVar("T", bound=dict)


def object_update(
    object_: T,
    path: str | list[str],
    updater: Callable[[object], object],
) -> T:
    """
    Sets a value at a deeply nested path from its current value, mutating the
    object. Reads the current value like `object_get` and writes the result
    like `object_set`.

    The updater receives None when the path is missing. Intermediate segments
    that are missing or not dicts are replaced with new dicts. An empty path
    list leaves the object unchanged and does not call the updater.

    Prototype pollution warning: this function does not filter out
    prototype-polluting keys ("__proto__", "constructor", "prototype").
    Sanitize user-controlled paths before calling.

    Args:
        object_ (T): Target object (mutated in place).
        path (str | list[str]): Dot-separated string or list of path segments.
        updater (Callable[[object], object]): Computes the new value from the
            current value.

    Returns:
        T: The same object reference, for chaining.

    Example:
        >>> object_update({"a": {"b": 1}}, "a.b", lambda value: value + 1)
        {'a': {'b': 2}}
        >>> object_update({}, ["a", "b"], lambda value: 0 if value is None else value)
        {'a': {'b': 0}}
    """
    segments = path_segments(path)
    if len(segments) == 0:
        return object_
    return object_set(object_, segments, updater(object_get(object_, segments)))
