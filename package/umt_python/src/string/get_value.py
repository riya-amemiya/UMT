import re
from typing import Any

# Pre-compiled regex pattern for array index notation to avoid recompilation
# per path segment.
_ARRAY_INDEX_PATTERN = re.compile(r"^(.+?)\[(-?\d+)\]$")

# Sentinel distinguishing "missing" from a stored ``None`` value, matching how
# the TypeScript source relies on JavaScript ``undefined``.
_MISSING = object()


def get_value(object_: Any, path: str) -> Any:  # noqa: ANN401
    """
    Retrieves a value from an object using a dot-notation path with array index
    support.

    Supports nested properties and array access with positive/negative indices.
    Out-of-bounds array indices resolve to ``None`` (mirroring JavaScript's
    ``undefined``).

    Args:
        object_: The object (mapping) to retrieve the value from.
        path: Dot-notation path (e.g. "user.name", "items[0]",
            "data[0].items[-1].name").

    Returns:
        The value at the specified path, or ``None`` if not found.

    Example:
        >>> get_value({"name": "Alice"}, "name")
        'Alice'
        >>> get_value({"user": {"name": "Bob"}}, "user.name")
        'Bob'
        >>> get_value({"items": ["A", "B", "C"]}, "items[0]")
        'A'
        >>> get_value({"items": ["A", "B", "C"]}, "items[-1]")
        'C'
    """
    segments: list[tuple[str, int | None]] = []

    for part in path.split("."):
        array_match = _ARRAY_INDEX_PATTERN.match(part)
        if array_match:
            key, index_string = array_match.group(1), array_match.group(2)
            segments.append((key, int(index_string)))
        else:
            segments.append((part, None))

    current: Any = object_

    for key, index in segments:
        if not isinstance(current, dict):
            return None

        current = current.get(key, _MISSING)
        if current is _MISSING:
            current = None

        if index is not None:
            if not isinstance(current, list):
                return None
            resolved = len(current) + index if index < 0 else index
            current = current[resolved] if 0 <= resolved < len(current) else None

    return current
