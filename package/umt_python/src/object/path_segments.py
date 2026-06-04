def path_segments(path: str | list[str]) -> list[str]:
    """
    Normalizes a property path into a list of segments.
    A dotted string is split on ".", while a list is returned unchanged.

    A list input is returned as-is (the same object, not a copy), mirroring the
    TypeScript `Object/pathSegments` behavior.

    Args:
        path (str | list[str]): Dot-separated string or list of path segments.

    Returns:
        list[str]: The path split into segments.

    Example:
        >>> path_segments("a.b.c")
        ['a', 'b', 'c']
        >>> path_segments(["a", "b"])
        ['a', 'b']
        >>> path_segments("a")
        ['a']
        >>> path_segments("")
        ['']
    """
    return path.split(".") if isinstance(path, str) else path
