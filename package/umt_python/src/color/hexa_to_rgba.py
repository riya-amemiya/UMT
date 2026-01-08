import re
from typing import Dict


def hexa_to_rgba(hex_code: str) -> Dict[str, float]:
    """
    Convert hexadecimal color code to RGBA color values.

    Args:
        hex_code: Hexadecimal color code (3, 6, or 8 digits with #)

    Returns:
        Dictionary with r, g, b as 0-255 and a as 0-1

    Raises:
        ValueError: If the hex code format is invalid

    Example:
        >>> hexa_to_rgba("#00000000")
        {'r': 0, 'g': 0, 'b': 0, 'a': 0.0}
        >>> hexa_to_rgba("#fff")
        {'r': 255, 'g': 255, 'b': 255, 'a': 1.0}
    """
    if not re.match(r"^#([0-9a-fA-F]{3}|[0-9a-fA-F]{6}|[0-9a-fA-F]{8})$", hex_code):
        raise ValueError("Invalid hex code")

    hex_value = hex_code.replace("#", "")

    if len(hex_value) == 3:
        hex_value = "".join(c + c for c in hex_value)

    r = int(hex_value[0:2], 16)
    g = int(hex_value[2:4], 16)
    b = int(hex_value[4:6], 16)
    alpha_value = int(hex_value[6:8], 16) if len(hex_value) == 8 else 255
    a = round(alpha_value / 255, 2)

    return {"r": r, "g": g, "b": b, "a": a}
