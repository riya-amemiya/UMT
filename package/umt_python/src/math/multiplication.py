from .get_decimal_length import get_decimal_length


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
        24
    """
    result = 1.0
    for number in numbers:
        n = 10 ** (get_decimal_length(result) + get_decimal_length(number))
        result_without_dot = int(str(result).replace(".", ""))
        number_without_dot = int(str(number).replace(".", ""))
        result = (result_without_dot * number_without_dot) / n
    return result
