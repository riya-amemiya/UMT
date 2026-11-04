import math

from .division import division
from .multiplication import multiplication

_PI_OVER_180 = division(math.pi, 180)


def deg_to_rad(x: float) -> float:
    """
    Converts degrees to radians.

    Args:
        x: Angle in degrees.

    Returns:
        Angle in radians.

    Example:
        >>> round(deg_to_rad(180), 10)
        3.1415926536
    """
    return multiplication(x, _PI_OVER_180)
