from collections.abc import Mapping

from .get_luminance import get_luminance


def get_contrast_ratio(
    color1: Mapping[str, int | float], color2: Mapping[str, int | float]
) -> float:
    """
    Calculate the WCAG 2.1 contrast ratio between two RGBA colors.

    Each color is a mapping with "r", "g", "b" (0-255) and an optional "a"
    (0-1), such as the dictionary returned by ``hexa_to_rgba``.

    Args:
        color1: First color
        color2: Second color

    Returns:
        Contrast ratio from 1 (identical luminance) to 21 (black on white).
        The argument order does not matter.

    Raises:
        ValueError: If any input values are out of their valid ranges

    Example:
        >>> get_contrast_ratio({"r": 0, "g": 0, "b": 0}, {"r": 255, "g": 255, "b": 255})
        21.0
    """
    luminance1 = get_luminance(
        color1["r"], color1["g"], color1["b"], color1.get("a", 1.0)
    )
    luminance2 = get_luminance(
        color2["r"], color2["g"], color2["b"], color2.get("a", 1.0)
    )
    return (max(luminance1, luminance2) + 0.05) / (min(luminance1, luminance2) + 0.05)
