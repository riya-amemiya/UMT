import math

from .get_decimal_length import get_decimal_length
from .js_round import js_round


def gcd(x: float, y: float, *z: float) -> float:
    """
    Greatest Common Divisor (GCD).

    Args:
        x: First number.
        y: Second number.
        *z: Additional numbers (optional).

    Returns:
        The GCD of all input numbers.

    Example:
        >>> gcd(12, 18)
        6.0
        >>> gcd(12, 18, 24)
        6.0
        >>> gcd(0.5, 0.25)
        0.25
    """
    numbers = (x, y, *z)

    if any(
        isinstance(number, float) and math.isfinite(number) and not number.is_integer()
        for number in numbers
    ):
        multiplier = 10 ** max(get_decimal_length(number) for number in numbers)
        return (
            _gcd_integer(*[float(number) * multiplier for number in numbers])
            / multiplier
        )

    return float(_gcd_integer(*numbers))


def _gcd_integer(x: float, y: float, *z: float) -> float:
    """Computes the GCD of the rounded absolute values of the inputs."""
    if math.isfinite(x) and not math.isfinite(y) and _round_absolute(x) == 0:
        return abs(y)
    if math.isfinite(y) and not math.isfinite(x) and _round_absolute(y) == 0:
        return abs(x)

    rounded_x = _round_absolute(x)
    rounded_y = _round_absolute(y)

    if rounded_x == 0:
        return rounded_y
    if rounded_y == 0:
        return rounded_x

    return math.gcd(rounded_x, rounded_y, *[_round_absolute(element) for element in z])


def _round_absolute(value: float) -> int:
    """Rounds half up to the nearest integer and drops the sign."""
    return abs(int(js_round(value)))
