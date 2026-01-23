def rgba_to_hexa(r: int, g: int, b: int, a: float = 1.0) -> str:
    """
    Convert RGBA color to hexadecimal color code.

    Args:
        r: Red value (0-255)
        g: Green value (0-255)
        b: Blue value (0-255)
        a: Alpha value (0-1)

    Returns:
        Hexadecimal color code including alpha channel

    Raises:
        ValueError: If any input values are out of their valid ranges

    Example:
        >>> rgba_to_hexa(0, 0, 0, 1)
        '#000000ff'
    """
    if r < 0 or r > 255 or g < 0 or g > 255 or b < 0 or b > 255 or a < 0 or a > 1:
        raise ValueError("Invalid rgba value")

    def to_hex(x: int) -> str:
        return format(x, "02x")

    return f"#{to_hex(r)}{to_hex(g)}{to_hex(b)}{to_hex(round(a * 255))}"
