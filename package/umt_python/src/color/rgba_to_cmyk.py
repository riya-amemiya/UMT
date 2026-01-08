from typing import Dict


def rgba_to_cmyk(r: int, g: int, b: int, a: float = 1.0) -> Dict[str, float]:
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

    r_prime = r / 255
    g_prime = g / 255
    b_prime = b / 255

    k = 1 - max(r_prime, g_prime, b_prime)

    if k == 1:
        c = 0
        m = 0
        y = 0
    else:
        c = (1 - r_prime - k) / (1 - k)
        m = (1 - g_prime - k) / (1 - k)
        y = (1 - b_prime - k) / (1 - k)

    return {
        "c": round(c * 100, 2),
        "m": round(m * 100, 2),
        "y": round(y * 100, 2),
        "k": round(k * 100, 2),
        "a": a,
    }
