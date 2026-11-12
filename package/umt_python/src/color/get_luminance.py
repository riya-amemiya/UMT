def _to_linear(channel: int | float) -> float:
    """Converts one sRGB channel (0-255) to its linear value (0-1)."""
    srgb = channel / 255
    return srgb / 12.92 if srgb <= 0.03928 else ((srgb + 0.055) / 1.055) ** 2.4


def get_luminance(
    r: int | float, g: int | float, b: int | float, a: float = 1.0
) -> float:
    """
    Calculate the WCAG 2.1 relative luminance of an RGBA color.

    Alpha is validated but does not affect the result.

    Args:
        r: Red value (0-255)
        g: Green value (0-255)
        b: Blue value (0-255)
        a: Alpha value (0-1)

    Returns:
        Relative luminance from 0 (black) to 1 (white)

    Raises:
        ValueError: If any input values are out of their valid ranges

    Example:
        >>> get_luminance(255, 255, 255)
        1.0
        >>> get_luminance(255, 0, 0)
        0.2126
    """
    if r < 0 or r > 255 or g < 0 or g > 255 or b < 0 or b > 255 or a < 0 or a > 1:
        raise ValueError("Invalid rgba value")

    return 0.2126 * _to_linear(r) + 0.7152 * _to_linear(g) + 0.0722 * _to_linear(b)
