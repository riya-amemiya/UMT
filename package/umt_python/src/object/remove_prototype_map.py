from collections.abc import Sequence
from typing import Any

from .remove_prototype import remove_prototype


def remove_prototype_map(objects: Sequence[dict[str, Any]]) -> list[dict[str, Any]]:
    """
    Creates a new array of objects with prototype-polluting properties removed
    from each item (shallow).

    Each element is sanitized with `remove_prototype`, so nested values are kept
    by reference. The input sequence and its elements are not mutated.

    Args:
        objects (Sequence[dict[str, Any]]): The objects to remove the
            prototype-polluting properties from.

    Returns:
        list[dict[str, Any]]: A new list with shallowly sanitized objects.

    Example:
        >>> remove_prototype_map(
        ...     [{"__proto__": {"x": True}, "a": 1}, {"prototype": 1, "c": 3}]
        ... )
        [{'a': 1}, {'c': 3}]
        >>> remove_prototype_map([])
        []
    """
    return [remove_prototype(object_) for object_ in objects]
