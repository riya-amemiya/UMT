from collections.abc import Sequence
from typing import Any

from .remove_prototype_deep import remove_prototype_deep


def remove_prototype_map_deep(
    objects: Sequence[dict[str, Any]],
) -> list[dict[str, Any]]:
    """
    Creates a new array of objects with prototype-polluting properties removed
    recursively from each item.

    Each element is sanitized with `remove_prototype_deep`, so nested plain
    objects and lists are deep-copied into fresh containers. The input sequence
    and its elements are not mutated.

    Args:
        objects (Sequence[dict[str, Any]]): The objects to remove the
            prototype-polluting properties from recursively.

    Returns:
        list[dict[str, Any]]: A new list with deeply sanitized objects.

    Example:
        >>> remove_prototype_map_deep(
        ...     [{"safe": {"__proto__": {"x": True}, "value": 1}}]
        ... )
        [{'safe': {'value': 1}}]
        >>> remove_prototype_map_deep([])
        []
    """
    return [remove_prototype_deep(object_) for object_ in objects]
