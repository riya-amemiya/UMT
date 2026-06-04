from typing import Any

from .object_is_plain import object_is_plain

_DANGEROUS_KEYS = ("__proto__", "constructor", "prototype")


def remove_prototype_deep(object_: dict[str, Any]) -> dict[str, Any]:
    """
    Creates a new object with the same properties as the given object, but with
    the prototype-polluting properties removed recursively.

    The keys "__proto__", "constructor" and "prototype" are excluded from nested
    objects and from objects found inside arrays (lists). Plain objects and lists
    are deep-copied into fresh containers; non-plain values (for example sets,
    dates, regex patterns) are passed through by reference. The input object is
    not mutated.

    Args:
        object_ (dict[str, Any]): The object to remove the prototype-polluting
            properties from.

    Returns:
        dict[str, Any]: A new object with the prototype-polluting properties
            removed recursively.

    Example:
        >>> remove_prototype_deep(
        ...     {"safe": {"nested": {"__proto__": {"x": True}, "value": 1}}}
        ... )
        {'safe': {'nested': {'value': 1}}}
        >>> remove_prototype_deep({"__proto__": 1, "a": {"__proto__": 2, "b": 3}})
        {'a': {'b': 3}}
    """
    result: dict[str, Any] = {}
    stack: list[tuple[Any, Any]] = [(object_, result)]

    while stack:
        source, destination = stack.pop()

        if isinstance(source, list):
            array: list[Any] = destination
            for value in source:
                if isinstance(value, list):
                    child_list: list[Any] = []
                    array.append(child_list)
                    stack.append((value, child_list))
                elif object_is_plain(value):
                    child_dict: dict[str, Any] = {}
                    array.append(child_dict)
                    stack.append((value, child_dict))
                else:
                    array.append(value)
            continue

        target: dict[str, Any] = destination
        for key in source:
            if key in _DANGEROUS_KEYS:
                continue
            value = source[key]
            if isinstance(value, list):
                child_list = []
                target[key] = child_list
                stack.append((value, child_list))
            elif object_is_plain(value):
                child_dict = {}
                target[key] = child_dict
                stack.append((value, child_dict))
            else:
                target[key] = value

    return result
