from typing import TypeVar

K = TypeVar("K")
V = TypeVar("V")


def invert(object_: dict[K, V]) -> dict[V, K]:
    """
    Creates a new object with keys and values swapped.
    If multiple keys share the same value, later entries overwrite earlier ones.

    The input object is not mutated.

    Args:
        object_ (dict[K, V]): Source object.

    Returns:
        dict[V, K]: A new object with keys and values swapped.

    Example:
        >>> invert({"a": 1, "b": 2})
        {1: 'a', 2: 'b'}
        >>> invert({"x": "X", "y": "Y"})
        {'X': 'x', 'Y': 'y'}
        >>> invert({"a": 1, "b": 1})
        {1: 'b'}
    """
    return {value: key for key, value in object_.items()}
