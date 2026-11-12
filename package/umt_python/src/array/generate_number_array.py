import math
import random

from ..math.addition import addition
from ..math.division import division
from ..math.multiplication import multiplication
from ..math.subtraction import subtraction

_MAX_SAFE_INTEGER = 2**53 - 1


def _is_safe_integer(value: int | float) -> bool:
    if isinstance(value, float) and not value.is_integer():
        return False
    return abs(value) <= _MAX_SAFE_INTEGER


def generate_number_array(
    length: int | float,
    min_val: int | float = 0,
    max_val: int | float | None = None,
    random_values: bool = False,
) -> list[int | float]:
    """
    Generates an array of numbers with the specified length.

    Args:
        length: The length of the array.
        min_val: The minimum value (default 0).
        max_val: The maximum value (default length - 1).
        random_values: Whether to generate random values (default False).

    Returns:
        Array of numbers.

    Example:
        >>> generate_number_array(5)
        [0, 1, 2, 3, 4]
        >>> generate_number_array(5, 10, 14)
        [10, 11, 12, 13, 14]
    """
    actual_length = math.floor(length)
    if actual_length <= 0:
        return []
    if actual_length == 1:
        return [min_val]

    actual_max = max_val if max_val is not None else actual_length - 1
    is_integer_inputs = _is_safe_integer(min_val) and _is_safe_integer(actual_max)

    if random_values:
        if is_integer_inputs:
            value_range = actual_max - min_val + 1
            return [
                math.floor(random.random() * value_range) + min_val
                for _ in range(actual_length)
            ]
        return [
            addition(
                math.floor(
                    multiplication(
                        random.random(), addition(subtraction(actual_max, min_val), 1)
                    )
                ),
                min_val,
            )
            for _ in range(actual_length)
        ]

    if is_integer_inputs:
        value_range = actual_max - min_val
        steps = actual_length - 1
        if value_range % steps == 0:
            step = value_range // steps
            return [min_val + index * step for index in range(actual_length)]

    step = division(subtraction(actual_max, min_val), actual_length - 1)
    return [
        addition(min_val, multiplication(index, step)) for index in range(actual_length)
    ]
