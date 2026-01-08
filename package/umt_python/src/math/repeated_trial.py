from typing import TypedDict

from .gcd import gcd
from .n_cr import n_cr


class ProbabilityFraction(TypedDict):
    x: float
    y: float


def repeated_trial(
    n: int,
    r: int,
    p: ProbabilityFraction,
) -> list[float]:
    """
    Calculate probability in repeated trials.

    Args:
        n: Number of trials.
        r: Number of successes.
        p: Probability fraction (x/y).

    Returns:
        Array containing [numerator, denominator].

    Example:
        >>> repeated_trial(5, 2, {'x': 1, 'y': 3})
        [80.0, 243.0]
    """
    x = n_cr(n, r)
    answer1 = x * (p["x"] ** r) * ((p["y"] - p["x"]) ** (n - r))
    answer2 = (p["y"] ** r) * (p["y"] ** (n - r))
    greatest_common_divisor = gcd(answer1, answer2)
    return [answer1 / greatest_common_divisor, answer2 / greatest_common_divisor]
