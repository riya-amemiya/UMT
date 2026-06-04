from typing import Any, TypeVar

T = TypeVar("T", bound=dict)


def object_set(object_: T, path: str | list[str], value: Any) -> T:  # noqa: ANN401
    """
    Sets a value at a deeply nested path on an object, mutating it.
    Creates intermediate objects as needed.

    A string path is split on "." into segments; a list path is used as-is.
    If an intermediate segment is missing or its value is not a dict, it is
    replaced with a new dict.

    Prototype pollution warning: this function does not filter out
    prototype-polluting keys ("__proto__", "constructor", "prototype").
    Sanitize user-controlled paths before calling.

    Args:
        object_ (T): Target object (mutated in place).
        path (str | list[str]): Dot-separated string or list of path segments.
        value (Any): Value to assign at the path.

    Returns:
        T: The same object reference, for chaining.

    Example:
        >>> object_set({}, "a.b.c", 1)
        {'a': {'b': {'c': 1}}}
        >>> object_set({}, ["a", "b"], 2)
        {'a': {'b': 2}}
        >>> object_set({"a": 5}, "a.b", 1)
        {'a': {'b': 1}}
    """
    segments = path.split(".") if isinstance(path, str) else path
    if len(segments) == 0:
        return object_
    current: dict[str, Any] = object_
    for index in range(len(segments) - 1):
        key = segments[index]
        next_value = current.get(key)
        if next_value is None or not isinstance(next_value, dict):
            current[key] = {}
        current = current[key]
    current[segments[-1]] = value
    return object_
