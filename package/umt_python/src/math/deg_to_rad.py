import math

from .division import division
from .multiplication import multiplication


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
    return multiplication(x, division(math.pi, 180))
