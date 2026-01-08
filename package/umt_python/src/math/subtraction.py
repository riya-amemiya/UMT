from .get_decimal_length import get_decimal_length
from .max_value import max_value
from .multiplication import multiplication


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
    result = 0.0
    for index, current in enumerate(numbers):
        if index == 0:
            result = current
        else:
            z = 10 ** max_value(get_decimal_length(result), get_decimal_length(current))
            result = (multiplication(result, z) - multiplication(current, z)) / z
    return result
