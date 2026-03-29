import re
from copy import copy
from datetime import date, datetime, time, timedelta
from typing import Any


def _clone_value(value: Any) -> Any:  # noqa: ANN401
    """Recursively clones a value."""
    if value is None or isinstance(value, (int, float, str, bool, bytes)):
        return value

    if isinstance(value, list):
        return [_clone_value(element) for element in value]

    if isinstance(value, tuple):
        return tuple(_clone_value(element) for element in value)

    if isinstance(value, (datetime, date, time, timedelta)):
        return copy(value)

    if isinstance(value, re.Pattern):
        return re.compile(value.pattern, value.flags)

    if isinstance(value, dict):
        result: dict[str, Any] = {}
        for key in value:
            result[key] = _clone_value(value[key])
        return result

    if isinstance(value, set):
        return {_clone_value(v) for v in value}

    if isinstance(value, frozenset):
        return frozenset(_clone_value(v) for v in value)

    return copy(value)


def deep_clone(value: Any) -> Any:  # noqa: ANN401
    """
    Creates a deep clone of the given value.

    Recursively copies nested structures including dicts, lists, tuples,
    sets, frozensets, datetime objects, and compiled regex patterns.
    Primitive values (int, float, str, bool, None, bytes) are returned as-is.

    Args:
        value: The value to deep clone.

    Returns:
        A deep clone of the input value.

    Example:
        >>> original = {"a": 1, "b": {"c": 2}}
        >>> cloned = deep_clone(original)
        >>> cloned["b"]["c"] = 99
        >>> original["b"]["c"]
        2
    """
    if value is None or isinstance(value, (int, float, str, bool, bytes)):
        return value
    return _clone_value(value)
