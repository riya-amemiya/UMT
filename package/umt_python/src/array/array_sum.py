import math

from ..math.number_to_js_string import number_to_js_string


def array_sum(x: list[int | float]) -> int | float:
    """
    Returns the sum of an array of numbers without floating point errors.

    Args:
        x: Array of numbers.

    Returns:
        Sum of the array elements.

    Example:
        >>> array_sum([1, 2, 3])
        6
        >>> array_sum([1.5, 2.5, 3.0])
        7.0
        >>> array_sum([0.1, 0.2])
        0.3
    """
    max_decimal = 0
    for num in x:
        if isinstance(num, int) or num.is_integer():
            continue
        decimal_length = len(number_to_js_string(num).partition(".")[2])
        max_decimal = max(max_decimal, decimal_length)

    if max_decimal == 0:
        result: int | float = 0
        for num in x:
            result += num
        return result

    scale = 10.0**max_decimal
    scaled = 0.0
    for num in x:
        scaled += _js_round(num * scale)
    return scaled / scale


def _js_round(value: float) -> float:
    """Rounds like JavaScript's Math.round (ties toward positive infinity)."""
    if not math.isfinite(value):
        return value
    floored = math.floor(value)
    return floored + 1 if value - floored >= 0.5 else floored
