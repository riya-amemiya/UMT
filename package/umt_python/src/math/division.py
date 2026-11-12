import math
from typing import Literal, overload

from .get_decimal_length import get_decimal_length
from .number_to_js_string import number_to_js_string


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

    x_int = _remove_decimal_point(abs_x)
    y_int = _remove_decimal_point(abs_y)

    scaling_factor = 10 ** (decimal_length_y - decimal_length_x)

    division_result = (x_int / y_int) * scaling_factor

    if is_floor:
        return sign * division_result

    int_quotient = (
        math.floor(division_result)
        if math.isfinite(division_result)
        else division_result
    )
    return [sign * int_quotient, x_int % y_int]


def _remove_decimal_point(value: float) -> float:
    """Reads the JavaScript string form of a number with its decimal point removed."""
    return float(number_to_js_string(value).replace(".", ""))
