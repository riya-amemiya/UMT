def cmyk_to_rgba(
    c: float, m: float, y: float, k: float, a: float = 1.0
) -> dict[str, float]:
    """
    Convert CMYK color values to RGBA color space.

    Args:
        c: Cyan percentage (0-100)
        m: Magenta percentage (0-100)
        y: Yellow percentage (0-100)
        k: Key/Black percentage (0-100)
        a: Alpha value (0-1)

    Returns:
        Dictionary with r, g, b as 0-255 and a as 0-1

    Example:
        >>> cmyk_to_rgba(100, 100, 0, 60.78)
        {'r': 0, 'g': 0, 'b': 100, 'a': 1.0}
    """
    clamped_c = max(0, min(100, c))
    clamped_m = max(0, min(100, m))
    clamped_y = max(0, min(100, y))
    clamped_k = max(0, min(100, k))

    c_percentage = clamped_c / 100
    m_percentage = clamped_m / 100
    y_percentage = clamped_y / 100
    k_percentage = clamped_k / 100

    r = 255 * (1 - c_percentage) * (1 - k_percentage)
    g = 255 * (1 - m_percentage) * (1 - k_percentage)
    b = 255 * (1 - y_percentage) * (1 - k_percentage)

    rounded_r = round(r)
    rounded_g = round(g)
    rounded_b = round(b)

    clamped_a = max(0, min(1, a))

    return {"r": rounded_r, "g": rounded_g, "b": rounded_b, "a": clamped_a}
