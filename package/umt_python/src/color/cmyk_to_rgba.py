from ..math.clamp import clamp
from ..math.division import division
from ..math.multiplication import multiplication
from ..math.round_of import round_of
from ..math.subtraction import subtraction


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
    c_percentage = division(clamp(c, 0, 100), 100)
    m_percentage = division(clamp(m, 0, 100), 100)
    y_percentage = division(clamp(y, 0, 100), 100)
    k_remainder = subtraction(1, division(clamp(k, 0, 100), 100))

    r = multiplication(255, subtraction(1, c_percentage), k_remainder)
    g = multiplication(255, subtraction(1, m_percentage), k_remainder)
    b = multiplication(255, subtraction(1, y_percentage), k_remainder)

    return {
        "r": round_of(r),
        "g": round_of(g),
        "b": round_of(b),
        "a": clamp(a, 0, 1),
    }
