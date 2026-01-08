from typing import Dict


def rgba_to_hsla(r: int, g: int, b: int, a: float = 1.0) -> Dict[str, float]:
    """
    Convert RGBA color values to HSLA color space.

    Args:
        r: Red value (0-255)
        g: Green value (0-255)
        b: Blue value (0-255)
        a: Alpha value (0-1)

    Returns:
        Dictionary with h as 0-360, s and l as 0-100, a as 0-1

    Raises:
        ValueError: If any input values are out of their valid ranges

    Example:
        >>> rgba_to_hsla(100, 100, 100, 1)
        {'h': 0, 's': 0, 'l': 39.22, 'a': 1}
    """
    if r < 0 or r > 255 or g < 0 or g > 255 or b < 0 or b > 255 or a < 0 or a > 1:
        raise ValueError("Invalid rgba value")

    r_prime = r / 255
    g_prime = g / 255
    b_prime = b / 255

    max_val = max(r_prime, g_prime, b_prime)
    min_val = min(r_prime, g_prime, b_prime)
    diff = max_val - min_val

    h = 0.0
    lightness = (max_val + min_val) / 2
    s = 0.0 if diff == 0 else diff / (1 - abs(max_val + min_val - 1))

    if diff != 0:
        if max_val == r_prime:
            h = ((g_prime - b_prime) / diff) + (6 if g_prime < b_prime else 0)
        elif max_val == g_prime:
            h = ((b_prime - r_prime) / diff) + 2
        elif max_val == b_prime:
            h = ((r_prime - g_prime) / diff) + 4
        h = h * 60

    return {
        "h": round(h, 2),
        "s": round(s * 100, 2),
        "l": round(lightness * 100, 2),
        "a": a,
    }
