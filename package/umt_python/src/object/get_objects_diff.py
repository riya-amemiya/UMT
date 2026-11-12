import math
from typing import Any, TypeVar

from ..validate.is_dictionary_object import is_dictionary_object

T = TypeVar("T", bound=dict[str, Any])


def _same_value_zero_key(value: object) -> object:
    """Builds a hashable key that groups values the way a JavaScript Map does."""
    if isinstance(value, bool):
        return (bool, value)
    if isinstance(value, float) and math.isnan(value):
        return (float, "nan")
    if isinstance(value, (int, float)):
        return (float, value)
    if isinstance(value, str) or value is None:
        return (type(value), value)
    return (object, id(value))


def get_objects_diff(obj: dict[str, Any], *objects: dict[str, Any]) -> dict[str, Any]:
    """
    Extract key-value pairs that appear in exactly one input object.

    A key-value pair is considered "shared" if the same key with the same value (===)
    exists in two or more objects. Only pairs unique to a single object are returned.
    When all values for a key are plain objects, the function recurses to find
    the diff subset. If the recursive result is empty, the key is excluded.
    When multiple unique pairs share the same key (different values),
    the last value wins.

    Keys appear in the result in first-seen order across the inputs. NaN values
    are treated as equal to each other, matching the TypeScript implementation.

    Args:
        obj: The first object.
        *objects: Additional objects to compare.

    Returns:
        Object containing only key-value pairs unique to one input.

    Example:
        >>> get_objects_diff({"a": 1, "b": 2}, {"b": 2, "c": 3})
        {'a': 1, 'c': 3}

        >>> get_objects_diff({"a": {"b": 1, "c": 2}, "d": 3}, {"a": {"b": 1, "d": 4}, "d": 3})
        {'a': {'c': 2, 'd': 4}}
    """
    all_objects = [obj, *objects]

    if len(all_objects) == 1:
        return obj.copy()

    all_keys: dict[str, None] = {}
    for o in all_objects:
        all_keys.update(dict.fromkeys(o))

    result: dict[str, Any] = {}

    for key in all_keys:
        values = [o[key] for o in all_objects if key in o]

        if len(values) == 1:
            result[key] = values[0]
            continue

        all_plain = all(is_dictionary_object(v) for v in values)

        if all_plain:
            nested = get_objects_diff(values[0], *values[1:])

            if nested:
                result[key] = nested
            continue

        value_keys = [_same_value_zero_key(value) for value in values]
        value_counts: dict[object, int] = {}
        for value_key in value_keys:
            value_counts[value_key] = value_counts.get(value_key, 0) + 1

        last_unique_value: Any = None
        has_unique = False

        for value, value_key in zip(values, value_keys, strict=True):
            if value_counts[value_key] == 1:
                last_unique_value = value
                has_unique = True

        if has_unique:
            result[key] = last_unique_value

    return result
