from typing import Any

_DANGEROUS_KEYS = ("__proto__", "constructor", "prototype")


def remove_prototype(object_: dict[str, Any]) -> dict[str, Any]:
    """
    Creates a new object with the same properties as the given object, but with
    the prototype-polluting properties removed.

    The keys "__proto__", "constructor" and "prototype" are excluded from the
    shallow copy. Nested values are copied by reference (shallow copy), and the
    input object is not mutated.

    Args:
        object_ (dict[str, Any]): The object to remove the prototype-polluting
            properties from.

    Returns:
        dict[str, Any]: A new object with the prototype-polluting properties
            removed.

    Example:
        >>> remove_prototype({"__proto__": {"polluted": True}, "a": 1, "b": 2})
        {'a': 1, 'b': 2}
        >>> remove_prototype({"constructor": 3, "a": 2, "prototype": 5, "b": 4})
        {'a': 2, 'b': 4}
    """
    return {key: value for key, value in object_.items() if key not in _DANGEROUS_KEYS}
