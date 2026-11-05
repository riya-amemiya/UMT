import math

from .division import division

_PI_OVER_180 = division(math.pi, 180)


def rad_to_deg(x: float) -> float:
    """
    Converts radians to degrees.

    Args:
        x: Angle in radians.

    Returns:
        Angle in degrees.

    Example:
        >>> rad_to_deg(math.pi)
        180.0
    """
    return division(x, _PI_OVER_180)
