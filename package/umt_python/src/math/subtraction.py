from decimal import Decimal


def subtraction(*numbers: float) -> float:
    """
    Performs subtraction with arbitrary number of arguments without floating point errors.

    Args:
        *numbers: Array of numbers to subtract.

    Returns:
        The result of the subtraction.

    Example:
        >>> subtraction(0.3, 0.1)
        0.2
        >>> subtraction(1, 0.1, 0.2)
        0.7
    """
    if not numbers:
        return 0.0

    result = Decimal(str(numbers[0]))
    for num in numbers[1:]:
        result -= Decimal(str(num))

    return float(result)
