from typing import Any
import math


def is_equal(a: Any, b: Any) -> bool:
    """
    Evaluates true strict equality.

    Uses similar semantics to JavaScript's Object.is(), treating NaN as equal
    to NaN and distinguishing between positive and negative zero.

    Args:
        a: First value to compare.
        b: Second value to compare.

    Returns:
        True if values are strictly equal, False otherwise.

    Example:
        >>> is_equal(1, 1)
        True
        >>> is_equal("test", "test")
        True
        >>> is_equal(float("nan"), float("nan"))
        True
        >>> is_equal(-0.0, 0.0)
        False
        >>> obj = {"a": 1}
        >>> is_equal(obj, obj)
        True
        >>> is_equal(obj, {"a": 1})
        False
    """
    if isinstance(a, float) and isinstance(b, float):
        if math.isnan(a) and math.isnan(b):
            return True
        if a == 0.0 and b == 0.0:
            return math.copysign(1.0, a) == math.copysign(1.0, b)
    return a is b
