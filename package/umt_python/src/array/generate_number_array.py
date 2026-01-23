import math
import random


def generate_number_array(
    length: int,
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

    Raises:
        ValueError: If min_val is greater than max_val.

    Example:
        >>> generate_number_array(5)
        [0, 1, 2, 3, 4]
        >>> generate_number_array(5, 10, 14)
        [10, 11, 12, 13, 14]
    """
    actual_length = math.floor(length)
    actual_max = max_val if max_val is not None else actual_length - 1

    if actual_length <= 0:
        return []

    if min_val > actual_max:
        raise ValueError("min should be less than or equal to max")

    if actual_length == 1:
        return [min_val]

    if random_values:
        return [
            min_val + math.floor(random.random() * (actual_max - min_val + 1))
            for _ in range(actual_length)
        ]

    step = (actual_max - min_val) / (actual_length - 1)
    return [min_val + i * step for i in range(actual_length)]
