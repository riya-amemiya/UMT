import math

from .division import division


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
    return division(x, division(math.pi, 180))
