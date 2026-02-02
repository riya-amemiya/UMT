import math


def gcd(x: float, y: float, *z: float) -> float:
    """
    Greatest Common Divisor (GCD).

    Args:
        x: First number.
        y: Second number.
        *z: Additional numbers (optional).

    Returns:
        The GCD of all input numbers.

    Example:
        >>> gcd(12, 18)
        6.0
        >>> gcd(12, 18, 24)
        6.0
        >>> gcd(0.5, 0.25)
        0.25
    """
    numbers = [x, y, *z]

    # Convert all inputs to (numerator, denominator) pairs
    fractions = []
    for num in numbers:
        if hasattr(num, "as_integer_ratio"):
            fractions.append(num.as_integer_ratio())
        else:
            # Fallback for types compatible with int/float but missing the method (e.g. strings)
            try:
                fractions.append(int(num).as_integer_ratio())
            except (ValueError, TypeError):
                fractions.append(float(num).as_integer_ratio())

    numerators = [n for n, _ in fractions]
    denominators = [d for _, d in fractions]

    # Optimization: if all are integers
    if all(d == 1 for d in denominators):
        return float(math.gcd(*numerators))

    common_denominator = math.lcm(*denominators)

    scaled_numerators = [
        num * (common_denominator // den)
        for num, den in zip(numerators, denominators)
    ]

    result_numerator = math.gcd(*scaled_numerators)

    return result_numerator / common_denominator
