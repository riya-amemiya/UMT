from typing import Any, TypeVar

from ..validate.is_dictionary_object import is_dictionary_object

T = TypeVar("T", bound=dict[str, Any])


def _is_strict_equal(a: object, b: object) -> bool:
    # Handle differing types
    if type(a) is not type(b):
        # JS treats int and float as 'number', so 1 === 1.0 is true.
        # Python treats them as distinct types, but 1 == 1.0 is true.
        # We allow int/float comparison, but strictly separate bool.
        if isinstance(a, (int, float)) and isinstance(b, (int, float)):
            if isinstance(a, bool) or isinstance(b, bool):
                return False
            return a == b
        return False

    # Same type
    # Primitives: int, float, str, bool, None
    if isinstance(a, (int, float, str, bool, type(None))):
        return a == b

    # Objects (list, dict, instances): use reference equality
    return a is b


def get_objects_diff(obj: T, *objects: dict[str, Any]) -> T:
    """
    Extract key-value pairs that appear in exactly one input object.

    A key-value pair is considered "shared" if the same key with the same value (===)
    exists in two or more objects. Only pairs unique to a single object are returned.
    When all values for a key are plain objects, the function recurses to find
    the diff subset. If the recursive result is empty, the key is excluded.
    When multiple unique pairs share the same key (different values),
    the last value wins.

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

    all_keys: set[str] = set()
    for o in all_objects:
        all_keys.update(o.keys())

    result: dict[str, Any] = {}

    for key in sorted(all_keys):
        values = [o[key] for o in all_objects if key in o]

        if len(values) == 1:
            result[key] = values[0]
            continue

        all_plain = all(is_dictionary_object(v) for v in values)

        if all_plain:
            # Type ignore because we know values are dicts due to check
            nested = get_objects_diff(values[0], *values[1:])  # type: ignore

            if nested:
                result[key] = nested
            continue

        last_unique_value: Any = None
        has_unique = False

        for value in values:
            count = 0
            for other in values:
                if _is_strict_equal(value, other):
                    count += 1

            if count == 1:
                last_unique_value = value
                has_unique = True

        if has_unique:
            result[key] = last_unique_value

    return result  # type: ignore
