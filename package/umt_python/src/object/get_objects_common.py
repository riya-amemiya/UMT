from typing import Any

from ..validate.is_dictionary_object import is_dictionary_object


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


def get_objects_common(obj: dict[str, Any], *objects: dict[str, Any]) -> dict[str, Any]:
    """
    Extract key-value pairs common to all input objects.

    A key-value pair is considered common when the key exists in every object
    and the value is strictly equal across all objects.
    When all values for a key are plain objects, the function recurses to find
    the common subset. If the recursive result is empty, the key is excluded.

    Args:
        obj: The first object.
        *objects: Additional objects to compare.

    Returns:
        Object containing only the key-value pairs shared by all inputs.

    Example:
        >>> get_objects_common({ "a": 1, "b": 2 }, { "a": 1, "c": 3 })
        {'a': 1}

        >>> get_objects_common({ "a": { "b": 1, "c": 2 }, "d": 3 }, { "a": { "b": 1, "d": 4 }, "d": 3 })
        {'a': {'b': 1}, 'd': 3}
    """
    if not objects:
        return obj.copy()

    result: dict[str, Any] = {}

    for key, value in obj.items():
        is_common = True
        all_plain_objects = is_dictionary_object(value)

        for other in objects:
            if key not in other:
                is_common = False
                break

            other_value = other[key]

            if not is_dictionary_object(other_value):
                all_plain_objects = False

            if not all_plain_objects and not _is_strict_equal(value, other_value):
                is_common = False
                break

        if not is_common:
            continue

        if all_plain_objects:
            nested = get_objects_common(value, *[o[key] for o in objects])  # type: ignore

            if nested:
                result[key] = nested
        else:
            result[key] = value

    return result
