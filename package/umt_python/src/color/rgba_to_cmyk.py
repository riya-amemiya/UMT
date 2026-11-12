import math

from ..math.division import division
from ..math.round_of import round_of
from ..math.subtraction import subtraction


def _channel(prime: float, k: float) -> float:
    """Resolves one CMY channel (0-1) from a normalized RGB channel and the key value."""
    value = division(subtraction(subtraction(1, prime), k), subtraction(1, k))
    return 0 if math.isnan(value) else value


def rgba_to_cmyk(r: int, g: int, b: int, a: float = 1.0) -> dict[str, float]:
    """
    Convert RGBA color to CMYK color model.

    Args:
        r: Red value (0-255)
        g: Green value (0-255)
        b: Blue value (0-255)
        a: Alpha value (0-1)

    Returns:
        Dictionary with c, m, y, k as percentages 0-100 and alpha channel

    Raises:
        ValueError: If any input values are out of their valid ranges

    Example:
        >>> rgba_to_cmyk(0, 0, 0, 1)
        {'c': 0, 'm': 0, 'y': 0, 'k': 100, 'a': 1}
    """
    if r < 0 or r > 255 or g < 0 or g > 255 or b < 0 or b > 255 or a < 0 or a > 1:
        raise ValueError("Invalid rgba value")

    r_prime = division(r, 255)
    g_prime = division(g, 255)
    b_prime = division(b, 255)

    k = subtraction(1, max(r_prime, g_prime, b_prime))

    return {
        "c": round_of(_channel(r_prime, k) * 100, 2),
        "m": round_of(_channel(g_prime, k) * 100, 2),
        "y": round_of(_channel(b_prime, k) * 100, 2),
        "k": round_of(k * 100, 2),
        "a": a,
    }
