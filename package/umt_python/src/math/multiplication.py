from decimal import Decimal


def multiplication(*numbers: float) -> float:
    """
    Performs multiplication without floating point errors for any number of arguments.

    Args:
        *numbers: Variable number of numbers to multiply.

    Returns:
        Product of all numbers.

    Example:
        >>> multiplication(0.1, 0.2, 0.3)
        0.006
        >>> multiplication(2, 3, 4)
        24.0
    """
    result = Decimal("1.0")
    for number in numbers:
        result *= Decimal(str(number))
    return float(result)
