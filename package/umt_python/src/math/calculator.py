import re
from collections.abc import Mapping

from .calculator_core import calculator_core
from .literal_expression import literal_expression

_WHITESPACE_RE = re.compile(r"\s+")


def calculator(
    expression: str,
    exchange: Mapping[str, float | str] | None = None,
) -> str:
    """
    Evaluates a mathematical expression or solves a simple single-variable
    equation. Supports parentheses, signs and currency conversion.

    Whitespace is removed before evaluation. When the expression contains an
    ``=`` sign it is treated as an equation, otherwise it is evaluated as an
    arithmetic expression.

    Args:
        expression: Mathematical expression or equation.
        exchange: Optional exchange rates for currency conversion.

    Returns:
        The calculation result as a string.

    Example:
        >>> calculator("1+2")
        '3'
        >>> calculator("(2+3)*4")
        '20'
        >>> calculator("x=5")
        '5'
    """
    clean_expression = _WHITESPACE_RE.sub("", expression)
    if "=" in clean_expression:
        return literal_expression(clean_expression)
    return calculator_core(clean_expression, exchange)
