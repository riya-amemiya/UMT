import math
from typing import Literal, overload

from .get_decimal_length import get_decimal_length


@overload
def division(x: float, y: float, is_floor: Literal[True] = ...) -> float: ...


@overload
def division(x: float, y: float, is_floor: Literal[False]) -> list[float]: ...


def division(x: float, y: float, is_floor: bool = True) -> float | list[float]:
    """
    Performs division without floating point errors.

    Args:
        x: Dividend.
        y: Divisor.
        is_floor: If True, returns quotient; if False, returns [quotient, remainder].

    Returns:
        Division result.

    Example:
        >>> division(0.1, 0.2)
        0.5
        >>> division(10, 3, False)
        [3, 1]
    """
    if y == 0:
        return float("nan") if is_floor else [float("nan"), float("nan")]

    sign = (1 if x >= 0 else -1) * (1 if y >= 0 else -1)
    abs_x = abs(x)
    abs_y = abs(y)

    decimal_length_x = get_decimal_length(abs_x)
    decimal_length_y = get_decimal_length(abs_y)

    x_int = int(str(abs_x).replace(".", ""))
    y_int = int(str(abs_y).replace(".", ""))

    scaling_factor = 10 ** (decimal_length_y - decimal_length_x)

    division_result = (x_int / y_int) * scaling_factor

    if is_floor:
        return sign * division_result

    int_quotient = math.floor(division_result)
    remainder = (x_int % y_int) / (10**decimal_length_x)
    return [sign * int_quotient, remainder]
