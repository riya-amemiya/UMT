from collections.abc import Callable, Mapping

from .calculator import calculator


def calculator_initialization(
    exchange: Mapping[str, float | str],
) -> Callable[[str], str]:
    """
    Creates a calculator function preconfigured with exchange rates.

    Args:
        exchange: Exchange rates object.

    Returns:
        A function that evaluates an expression using the configured rates.

    Example:
        >>> calculator_initialization({"$": 100})("$1")
        '100'
        >>> calculator_initialization({"EUR": 1.2})("EUR5 + 10")
        '16'
    """

    def calculate(x: str) -> str:
        return calculator(x, exchange)

    return calculate
