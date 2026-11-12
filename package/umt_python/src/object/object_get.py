from typing import Any


def object_get(obj: Any, path: str | list[str], default_value: Any = None) -> Any:  # noqa: ANN401
    """
    Reads a deeply nested property by string or string-list path.
    Returns the default value if any segment is missing along the path.

    Mirrors the TypeScript `Object/get` semantics from package/main, with one
    Python-idiomatic adaptation: TS distinguishes `undefined` (missing) from
    `null` (explicit) and only swaps in the default for `undefined`. Python
    collapses both to `None`, so we preserve the distinction structurally by
    checking key presence in the parent dict (mirroring `object_has`). An
    explicit `None` value at the final segment is returned as-is, while a
    missing segment returns `default_value`.

    Lists are traversed by canonical decimal index segments (for example
    "0"), mirroring index access on JavaScript arrays.

    Args:
        obj: Source object (dict-like) or any value
        path: Dot-separated string or list of path segments
        default_value: Returned when any segment along the path is missing

    Returns:
        The resolved value, or default_value when any segment is missing

    Example:
        >>> object_get({"a": {"b": {"c": 1}}}, "a.b.c")
        1
        >>> object_get({"a": {"b": 1}}, ["a", "x"], 0)
        0
        >>> object_get({"a": None}, "a.b", 7)
        7
        >>> object_get({"a": [10, 20]}, "a.1")
        20
    """
    segments = path.split(".") if isinstance(path, str) else path
    current: Any = obj
    for key in segments:
        if isinstance(current, dict):
            if key not in current:
                return default_value
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
            return default_value
    return current
