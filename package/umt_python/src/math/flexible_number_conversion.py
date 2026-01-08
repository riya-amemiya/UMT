
from ..validate import is_value_nan


def flexible_number_conversion(value: object) -> float:
    """
    Flexible function to convert various inputs to numbers whenever possible.

    Args:
        value: Value to convert (any type).

    Returns:
        Converted number, or NaN if conversion is not possible.

    Example:
        >>> flexible_number_conversion(123)
        123
        >>> flexible_number_conversion("456")
        456.0
        >>> flexible_number_conversion("78.9")
        78.9
        >>> flexible_number_conversion("3.14e2")
        314.0
        >>> flexible_number_conversion("0xFF")
        255.0
        >>> flexible_number_conversion("42px")
        42.0
        >>> flexible_number_conversion("")
        0
        >>> math.isnan(flexible_number_conversion("not a number"))
        True
    """
    # Return NaN for objects (dict, list, etc.)
    if isinstance(value, (dict, list)):
        return float("nan")

    # Handle special cases
    if value is None or value == "":
        return 0

    # Handle values already of type number
    if isinstance(value, (int, float)) and not is_value_nan(value):
        return value

    # Convert to string and process
    string_value = str(value).strip().lower()

    # Handle infinity
    if string_value in ("infinity", "+infinity"):
        return float("inf")
    if string_value == "-infinity":
        return float("-inf")

    # Handle special base notations (hex, octal, binary)
    if string_value.startswith("0x"):
        try:
            return float(int(string_value, 16))
        except ValueError:
            pass
    if string_value.startswith("0o"):
        try:
            return float(int(string_value, 8))
        except ValueError:
            pass
    if string_value.startswith("0b"):
        try:
            return float(int(string_value, 2))
        except ValueError:
            pass

    # Parse as floating point number
    try:
        float_value = float(string_value)
        if not is_value_nan(float_value):
            return float_value
    except ValueError:
        pass

    # Try to extract leading number (like "42px" -> 42)
    result = ""
    for char in string_value:
        if char.isdigit() or char == "." or (char == "-" and not result):
            result += char
        else:
            break

    if result and result != "-" and result != ".":
        try:
            return float(result)
        except ValueError:
            pass

    # When conversion is not possible
    return float("nan")
