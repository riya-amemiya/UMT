from collections.abc import Callable
from typing import TypeVar

K = TypeVar("K", bound=str)


def unit_converter_initialization(
    to_base_unit_ratios: dict[str, float],
) -> Callable[[float, str, str], float]:
    """
    Unit converter initialization function.
    Creates a converter function that can convert between different units using base unit ratios.

    Args:
        to_base_unit_ratios: Dictionary containing conversion ratios to the base unit

    Returns:
        A function that converts values between units

    Example:
        >>> length_converter = unit_converter_initialization({
        ...     "meters": 1,
        ...     "kilometers": 1000,
        ...     "centimeters": 0.01
        ... })
        >>> length_converter(5, "kilometers", "meters")
        5000.0
    """

    def converter(value: float, from_unit: str, to_unit: str) -> float:
        return (value / to_base_unit_ratios[from_unit]) * to_base_unit_ratios[to_unit]

    return converter
