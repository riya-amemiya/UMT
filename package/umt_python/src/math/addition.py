from .get_decimal_length import get_decimal_length
from .max_value import max_value
from .multiplication import multiplication


def addition(*numbers: float) -> float:
    """
    Addition without floating point errors.

    Args:
        *numbers: Variable number of numbers to add.

    Returns:
        Sum of the numbers.

    Example:
        >>> addition(0.1, 0.2)
        0.3
        >>> addition(1, 2, 3)
        6
    """
    z = 10 ** max_value(*[get_decimal_length(element) for element in numbers])
    total = sum(multiplication(current, z) for current in numbers)
    return total / z
