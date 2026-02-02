import math
from collections import Counter
from dataclasses import dataclass


@dataclass
class IsDeepEqualOptions:
    """
    Options for is_deep_equal comparison.

    Attributes:
        strict_order: Whether to ignore array order when comparing arrays.
    """

    strict_order: bool = True


def is_deep_equal(
    a: object, b: object, options: IsDeepEqualOptions | None = None
) -> bool:
    """
    Performs a deep equality comparison between two values.

    Args:
        a: First value to compare.
        b: Second value to compare.
        options: Comparison options.

    Returns:
        True if values are deeply equal, False otherwise.

    Example:
        >>> is_deep_equal({"a": 1, "b": [2, 3]}, {"b": [2, 3], "a": 1})
        True
        >>> is_deep_equal([1, 2, 3], [3, 2, 1])
        False
        >>> is_deep_equal([1, 2, 3], [3, 2, 1], IsDeepEqualOptions(strict_order=False))
        True
        >>> is_deep_equal({1, 2}, {2, 1})
        True
    """
    if options is None:
        options = IsDeepEqualOptions()
    strict_order = options.strict_order
    visited: set[int] = set()

    def compare(x: object, y: object) -> bool:
        if x is y:
            return True

        if x is None or y is None:
            return x is y

        if type(x) is not type(y):
            return False

        if isinstance(x, (int, float, str, bool)):
            if (
                isinstance(x, float)
                and isinstance(y, float)
                and math.isnan(x)
                and math.isnan(y)
            ):
                return True
            return x == y

        id_x = id(x)
        id_y = id(y)
        if id_x in visited or id_y in visited:
            return True
        visited.add(id_x)
        visited.add(id_y)

        if isinstance(x, list) and isinstance(y, list):
            if len(x) != len(y):
                return False

            if strict_order:
                for i, elem in enumerate(x):
                    if not compare(elem, y[i]):
                        return False
            else:
                try:

                    def _make_hashable_key(item: object) -> tuple[type, object]:
                        t = type(item)
                        if t is float and math.isnan(item):  # type: ignore
                            return (float, "NaN")
                        return (t, item)

                    return Counter(_make_hashable_key(i) for i in x) == Counter(
                        _make_hashable_key(i) for i in y
                    )
                except TypeError:
                    pass

                y_copy = list(y)
                for item_x in x:
                    found = False
                    for i, item_y in enumerate(y_copy):
                        if compare(item_x, item_y):
                            y_copy.pop(i)
                            found = True
                            break
                    if not found:
                        return False
                return len(y_copy) == 0
            return True

        if isinstance(x, set) and isinstance(y, set):
            if len(x) != len(y):
                return False

            for item in x:
                found = False
                for other_item in y:
                    if compare(item, other_item):
                        found = True
                        break
                if not found:
                    return False
            return True

        if isinstance(x, dict) and isinstance(y, dict):
            if len(x) != len(y):
                return False

            for key in x:
                if key not in y:
                    return False

            return all(compare(x[key], y[key]) for key in x)

        if isinstance(x, (bytes, bytearray)) and isinstance(y, (bytes, bytearray)):
            return x == y

        return x == y

    return compare(a, b)
