from ..math.addition import addition


def to_kelvin(celsius: float) -> float:
    """
    Converts temperature from Celsius to Kelvin.

    Args:
        celsius: Temperature in Celsius.

    Returns:
        Temperature in Kelvin.

    Example:
        >>> to_kelvin(26.85)
        300.0
        >>> to_kelvin(0)
        273.15
        >>> to_kelvin(-273.15)
        0.0
    """
    return addition(celsius, 273.15)
