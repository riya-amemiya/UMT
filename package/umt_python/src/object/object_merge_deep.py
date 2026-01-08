from typing import Dict, Any


def _is_plain_object(value: Any) -> bool:
    """Checks if a value is a plain dictionary."""
    return isinstance(value, dict)


def object_merge_deep(
    target: Dict[str, Any], *sources: Dict[str, Any]
) -> Dict[str, Any]:
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
    if not sources:
        return target

    sources_list = list(sources)
    source = sources_list.pop(0)

    if _is_plain_object(target) and _is_plain_object(source):
        result = target.copy()

        for key, source_value in source.items():
            target_value = result.get(key)

            if isinstance(target_value, dict) and isinstance(source_value, dict):
                result[key] = object_merge_deep(target_value, source_value)
            else:
                result[key] = source_value

        return object_merge_deep(result, *sources_list)

    return object_merge_deep(source, *sources_list)
