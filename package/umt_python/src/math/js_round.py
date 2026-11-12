import math


def js_round(value: float) -> float:
    """
    Rounds like JavaScript ``Math.round``: ties go toward positive infinity.

    Integers, NaN and infinities are returned unchanged.

    Args:
        value: Number to round.

    Returns:
        The rounded number.

    Example:
        >>> js_round(2.5)
        3
        >>> js_round(-2.5)
        -2
        >>> js_round(0.49999999999999994)
        0
    """
    if isinstance(value, int) or not math.isfinite(value):
        return value
    floored = math.floor(value)
    return floored + 1 if value - floored >= 0.5 else floored
