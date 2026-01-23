from typing import Any


def object_pick_deep(obj: dict[str, Any], *keys: str) -> dict[str, Any]:
    """
    Creates a new object by deeply selecting properties from the source object based on specified keys.

    Args:
        obj: The source object to extract properties from
        *keys: Property keys to extract. Can use dot notation for nested properties.

    Returns:
        A new object containing only the specified properties

    Example:
        >>> obj = {"a": {"b": {"c": 1, "d": 2}, "e": 3}, "f": 4}
        >>> object_pick_deep(obj, "a.b.c", "f")
        {'a': {'b': {'c': 1}}, 'f': 4}
    """
    result: dict[str, Any] = {}

    for key in keys:
        parts = key.split(".")
        current = obj.copy()
        target = result

        for i, part in enumerate(parts):
            if current and isinstance(current, dict) and part in current:
                if i == len(parts) - 1:
                    target[part] = current[part]
                else:
                    if part not in target:
                        target[part] = {}
                    current = current[part]
                    target = target[part]

    return result
