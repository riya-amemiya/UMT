from typing import Any


def _is_plain_object(value: Any) -> bool:  # noqa: ANN401
    """Checks if a value is a plain dictionary."""
    return isinstance(value, dict)


def object_merge_deep(target: Any, *sources: Any) -> Any:  # noqa: ANN401
    """
    Deeply merges multiple objects into a single object.

    Args:
        target: The target object to merge into
        *sources: The source objects to merge from

    Returns:
        The deeply merged object

    Example:
        >>> object_merge_deep({"a": {"b": 1}}, {"a": {"c": 2}})
        {'a': {'b': 1, 'c': 2}}
    """
    result = target
    is_owned = False

    for source in sources:
        if _is_plain_object(result) and _is_plain_object(source):
            if not is_owned:
                result = result.copy()
                is_owned = True

            for key, source_value in source.items():
                target_value = result.get(key)

                if isinstance(target_value, dict) and isinstance(source_value, dict):
                    result[key] = object_merge_deep(target_value, source_value)
                else:
                    result[key] = source_value
        else:
            result = source
            is_owned = False

    return result
