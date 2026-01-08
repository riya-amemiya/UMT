from typing import Dict, Any


def object_merge(target: Dict[str, Any], *sources: Dict[str, Any]) -> Dict[str, Any]:
    """
    Merges multiple objects into a single object (shallow merge).

    Args:
        target: The target object to merge into
        *sources: The source objects to merge from

    Returns:
        The merged object

    Example:
        >>> object_merge({"a": 1}, {"b": 2}, {"c": 3})
        {'a': 1, 'b': 2, 'c': 3}
    """
    result = target.copy()
    for source in sources:
        result.update(source)
    return result
