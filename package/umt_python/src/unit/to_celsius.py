from ..math.subtraction import subtraction


def to_celsius(kelvin: float) -> float:
    """
    Converts temperature from Kelvin to Celsius.

    Args:
        kelvin: Temperature in Kelvin.

    Returns:
        Temperature in Celsius.

    Example:
        >>> to_celsius(300)
        26.85
        >>> to_celsius(273.15)
        0.0
        >>> to_celsius(0)
        -273.15
    """
    return subtraction(kelvin, 273.15)
