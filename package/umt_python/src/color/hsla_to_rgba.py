from ..math.addition import addition
from ..math.clamp import clamp
from ..math.division import division
from ..math.multiplication import multiplication
from ..math.round_of import round_of
from ..math.subtraction import subtraction

_ONE_SIXTH = division(1, 6)
_ONE_THIRD = division(1, 3)
_ONE_HALF = division(1, 2)
_TWO_THIRDS = division(2, 3)


def _hue_to_rgb(p: float, q: float, t: float) -> float:
    """Resolves one RGB channel (0-1) from the HSL intermediates p, q and the hue offset t."""
    t_adjusted = addition(t, 1) if t < 0 else t
    if t > 1:
        t_adjusted = subtraction(t, 1)
    if t_adjusted < _ONE_SIXTH:
        return addition(
            p, multiplication(subtraction(q, p), multiplication(6, t_adjusted))
        )
    if t_adjusted < _ONE_HALF:
        return q
    if t_adjusted < _TWO_THIRDS:
        return addition(
            p,
            multiplication(
                subtraction(q, p),
                multiplication(subtraction(_TWO_THIRDS, t_adjusted), 6),
            ),
        )
    return p


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
        {'r': 63.75, 'g': 191.25, 'b': 63.75, 'a': 1}
    """
    if h < 0 or h > 360:
        raise ValueError("Hue must be between 0 and 360 degrees")
    if s < 0 or s > 100:
        raise ValueError("Saturation must be between 0 and 100 percent")
    if lightness < 0 or lightness > 100:
        raise ValueError("Lightness must be between 0 and 100 percent")
    if a < 0 or a > 1:
        raise ValueError("Alpha must be between 0 and 1")

    hue = division(division(h, 360, False)[1], 360)
    saturation = division(clamp(s, 0, 100), 100)
    lightness_normalized = division(clamp(lightness, 0, 100), 100)

    if saturation == 0:
        r = g = b = lightness_normalized
    else:
        q = (
            multiplication(lightness_normalized, addition(1, saturation))
            if lightness_normalized < 0.5
            else subtraction(
                addition(lightness_normalized, saturation),
                multiplication(lightness_normalized, saturation),
            )
        )
        p = subtraction(multiplication(2, lightness_normalized), q)

        r = _hue_to_rgb(p, q, addition(hue, _ONE_THIRD))
        g = _hue_to_rgb(p, q, hue)
        b = _hue_to_rgb(p, q, subtraction(hue, _ONE_THIRD))

    return {
        "r": round_of(multiplication(r, 255), 2),
        "g": round_of(multiplication(g, 255), 2),
        "b": round_of(multiplication(b, 255), 2),
        "a": clamp(a, 0, 1),
    }
