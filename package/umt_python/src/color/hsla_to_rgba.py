def hsla_to_rgba(
    h: float, s: float, lightness: float, a: float = 1.0
) -> dict[str, float]:
    """
    Convert HSLA color values to RGBA color space.

    Args:
        h: Hue angle in degrees (0-360)
        s: Saturation percentage (0-100)
        lightness: Lightness percentage (0-100)
        a: Alpha value (0-1)

    Returns:
        Dictionary with r, g, b as 0-255 and a as 0-1

    Raises:
        ValueError: If any input values are out of their valid ranges

    Example:
        >>> hsla_to_rgba(120, 50, 50, 1)
        {'r': 64, 'g': 191, 'b': 64, 'a': 1.0}
    """
    if h < 0 or h > 360:
        raise ValueError("Hue must be between 0 and 360 degrees")
    if s < 0 or s > 100:
        raise ValueError("Saturation must be between 0 and 100 percent")
    if lightness < 0 or lightness > 100:
        raise ValueError("Lightness must be between 0 and 100 percent")
    if a < 0 or a > 1:
        raise ValueError("Alpha must be between 0 and 1")

    hue = (h % 360) / 360
    saturation = max(0, min(s, 100)) / 100
    lightness_normalized = max(0, min(lightness, 100)) / 100

    r = 0.0
    g = 0.0
    b = 0.0

    if saturation == 0:
        r = g = b = lightness_normalized
    else:

        def hue_to_rgb(p: float, q: float, t: float) -> float:
            t_adjusted = t
            if t < 0:
                t_adjusted = t + 1
            if t > 1:
                t_adjusted = t - 1
            if t_adjusted < 1 / 6:
                return p + (q - p) * 6 * t_adjusted
            if t_adjusted < 1 / 2:
                return q
            if t_adjusted < 2 / 3:
                return p + (q - p) * (2 / 3 - t_adjusted) * 6
            return p

        q = (
            lightness_normalized * (1 + saturation)
            if lightness_normalized < 0.5
            else lightness_normalized + saturation - lightness_normalized * saturation
        )
        p = 2 * lightness_normalized - q

        r = hue_to_rgb(p, q, hue + 1 / 3)
        g = hue_to_rgb(p, q, hue)
        b = hue_to_rgb(p, q, hue - 1 / 3)

    rounded_r = round(r * 255, 2)
    rounded_g = round(g * 255, 2)
    rounded_b = round(b * 255, 2)
    clamped_a = max(0, min(1, a))

    return {"r": rounded_r, "g": rounded_g, "b": rounded_b, "a": clamped_a}
