from .is_double import is_double


def _gcd_integer(x: int, y: int, *z: int) -> int:
    """Internal function to calculate GCD for integers."""
    copy_x = abs(x)
    copy_y = abs(y)
    copy_z = [abs(element) for element in z]

    if copy_x == 0:
        return copy_y
    if copy_y == 0:
        return copy_x

    copy_x, copy_y = max(copy_x, copy_y), min(copy_x, copy_y)

    # Euclidean algorithm
    r = copy_y % copy_x
    while r != 0:
        copy_y = copy_x
        copy_x = r
        r = copy_y % copy_x

    if len(copy_z) > 0:
        for element in copy_z:
            copy_x = _gcd_integer(copy_x, element)

    return copy_x


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
        6
        >>> gcd(12, 18, 24)
        6
        >>> gcd(0.5, 0.25)
        0.25
    """
    all_numbers = [x, y, *z]

    has_decimals = any(is_double(num, False) for num in all_numbers)

    if has_decimals:

        def get_decimal_places(num: float) -> int:
            string_val = str(num)
            return len(string_val.split(".")[1]) if "." in string_val else 0

        max_decimal_places = max(get_decimal_places(num) for num in all_numbers)
        multiplier = 10**max_decimal_places

        scaled_numbers = [round(num * multiplier) for num in all_numbers]
        result = _gcd_integer(scaled_numbers[0], scaled_numbers[1], *scaled_numbers[2:])

        return result / multiplier

    return float(_gcd_integer(int(x), int(y), *[int(num) for num in z]))
