from typing import TypeVar

T = TypeVar("T")


def object_pick(obj: dict[str, T], *keys: str) -> dict[str, T]:
    """
    Creates a new object with only the specified properties from the source object.

    Args:
        obj: The source object to extract properties from
        *keys: The property keys to extract

    Returns:
        A new object containing only the specified properties

    Example:
        >>> object_pick({"id": 1, "name": "Alice", "age": 30}, "id", "name")
        {'id': 1, 'name': 'Alice'}
    """
    result: dict[str, T] = {}
    for key in keys:
        if key in obj:
            result[key] = obj[key]
    return result
