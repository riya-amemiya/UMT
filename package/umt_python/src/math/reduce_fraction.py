from typing import TypedDict

from .gcd import gcd


class ReducedFraction(TypedDict):
    x: float
    y: float
    gcd: float


def reduce_fraction(x: float, y: float) -> ReducedFraction:
    """
    Reduces a fraction to its lowest terms.

    Args:
        x: Numerator.
        y: Denominator.

    Returns:
        Reduced fraction and the GCD.

    Example:
        >>> reduce_fraction(2, 4)
        {'x': 1, 'y': 2, 'gcd': 2}
    """
    if y == 0:
        return {"x": float("nan"), "y": float("nan"), "gcd": float("nan")}
    if x == 0:
        return {"x": 0, "y": 1, "gcd": abs(y)}
    gcd_value = gcd(abs(x), abs(y))
    sign = -1 if y < 0 else 1
    return {
        "x": (x / gcd_value) * sign,
        "y": abs(y / gcd_value),
        "gcd": gcd_value,
    }
