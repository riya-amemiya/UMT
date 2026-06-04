import math
import re
from collections.abc import Mapping

from ..tool.escape_regexp import escape_regexp
from ..validate.is_number import is_number
from .addition import addition
from .convert_currency import convert_currency
from .division import division
from .multiplication import multiplication
from .number_to_js_string import number_to_js_string
from .subtraction import subtraction

# Sentinel value matching JavaScript's ``Number.NaN.toString()``.
# The resolver helpers return this string to signal "no progress was made",
# which makes the main loop bail out and return the expression unchanged.
_NAN_SENTINEL = "NaN"

_PARENTHESES_RE = re.compile(r"\((-?\d+(?:\.\d+)?)([*+/-])(-?\d+(?:\.\d+)?)\)")
_MUL_EXP_RE = re.compile(r"(.*?)(-?\d+(?:\.\d+)?)([*^])(-?\d+(?:\.\d+)?)$")
_DIV_RE = re.compile(r"(-?\d+(?:\.\d+)?)/(-?\d+(?:\.\d+)?)")
_ADD_SUB_RE = re.compile(r"(-?\d+(?:\.\d+)?)(\+|-)(-?\d+(?:\.\d+)?)")
_PAREN_STRIP_RE = re.compile(r"[()]")

_currency_regex_cache: dict[str, re.Pattern[str]] = {}


def calculator_core(
    expression: str,
    currency_exchange: Mapping[str, float | str] | None = None,
) -> str:
    """
    Evaluates a mathematical expression string, resolving parentheses,
    exponentiation, multiplication, division, addition and subtraction.

    Operations are resolved iteratively. When an expression cannot be reduced
    any further (incomplete or invalid input), the expression is returned as-is.

    Args:
        expression: Mathematical expression with no whitespace.
        currency_exchange: Optional mapping of currency symbols to rates.

    Returns:
        The calculation result as a string, or the original expression when it
        cannot be evaluated.

    Example:
        >>> calculator_core("1+1")
        '2'
        >>> calculator_core("2^3")
        '8'
        >>> calculator_core("5/0")
        '5/0'
    """
    if expression == "":
        return ""

    sanitized_expression = _sanitize_signs(expression)

    while True:
        if currency_exchange:
            sanitized_expression = _apply_currency_exchange(
                sanitized_expression, currency_exchange
            )

        if _contains_parentheses(sanitized_expression):
            temporary = _resolve_parentheses(sanitized_expression)
            if temporary == _NAN_SENTINEL:
                return sanitized_expression
            sanitized_expression = temporary

        elif _contains_mul_exp(sanitized_expression):
            temporary = _resolve_mul_exp(sanitized_expression)
            if temporary == _NAN_SENTINEL:
                return sanitized_expression
            sanitized_expression = temporary

        elif _contains_div(sanitized_expression):
            temporary = _resolve_div(sanitized_expression)
            if temporary == _NAN_SENTINEL:
                return sanitized_expression
            sanitized_expression = temporary

        elif _contains_add_sub(sanitized_expression) and not is_number(
            sanitized_expression
        ):
            temporary = _resolve_add_sub(sanitized_expression)
            if temporary == _NAN_SENTINEL:
                return sanitized_expression
            sanitized_expression = temporary

        else:
            number = _to_number(sanitized_expression)
            if not math.isnan(number):
                # Handle floating point precision issues.
                rounded = _js_round(number * 1e10) / 1e10
                return number_to_js_string(rounded)
            return sanitized_expression


def _sanitize_signs(expr: str) -> str:
    return (
        expr.replace("--", "+")
        .replace("++", "+")
        .replace("+-", "+0-")
        .replace("-+", "+0-")
    )


def _get_currency_regex(currency_symbol: str) -> re.Pattern[str]:
    cached = _currency_regex_cache.get(currency_symbol)
    if cached is not None:
        return cached
    regex = re.compile(f"{escape_regexp(currency_symbol)}([0-9]+)")
    _currency_regex_cache[currency_symbol] = regex
    return regex


def _apply_currency_exchange(expr: str, rates: Mapping[str, float | str]) -> str:
    return_expr = expr
    for index in rates:
        if index in return_expr:
            match = _get_currency_regex(index).search(return_expr)
            if match:
                return_expr = return_expr.replace(
                    match.group(0), convert_currency(match.group(0), rates), 1
                )
    return return_expr


def _contains_parentheses(expr: str) -> bool:
    return "(" in expr or ")" in expr


def _resolve_parentheses(expr: str) -> str:
    match = _PARENTHESES_RE.search(expr)
    if match:
        inner = _PAREN_STRIP_RE.sub("", match.group(0))
        return expr.replace(match.group(0), calculator_core(inner), 1)
    return _NAN_SENTINEL


def _contains_mul_exp(expr: str) -> bool:
    return "^" in expr or "*" in expr


def _contains_div(expr: str) -> bool:
    return "/" in expr


def _resolve_mul_exp(expr: str) -> str:
    match = _MUL_EXP_RE.search(expr)
    if match:
        left = _to_number(match.group(2))
        right = _to_number(match.group(4))
        if match.group(3) == "^":
            result = _js_pow(left, right)
            return f"{match.group(1)}{number_to_js_string(result)}"
        result = multiplication(left, right)
        return f"{match.group(1)}{number_to_js_string(result)}"
    return _NAN_SENTINEL


def _resolve_div(expr: str) -> str:
    match = _DIV_RE.search(expr)
    if match:
        result = division(_to_number(match.group(1)), _to_number(match.group(2)))
        return expr.replace(match.group(0), number_to_js_string(result), 1)
    return _NAN_SENTINEL


def _contains_add_sub(expr: str) -> bool:
    return "+" in expr or "-" in expr


def _resolve_add_sub(expr: str) -> str:
    match = _ADD_SUB_RE.search(expr)
    if match:
        left = _to_number(match.group(1))
        right = _to_number(match.group(3))
        if match.group(2) == "+":
            result = addition(left, right)
        else:
            result = subtraction(left, right)
        return expr.replace(match.group(0), number_to_js_string(result), 1)
    return _NAN_SENTINEL


def _to_number(value: str) -> float:
    """
    Mimics JavaScript's ``Number(value)`` coercion for a string, returning NaN
    for non-numeric strings (including ``"Infinity"`` handling).
    """
    stripped = value.strip()
    if stripped == "":
        return 0.0
    if stripped in ("Infinity", "+Infinity"):
        return float("inf")
    if stripped == "-Infinity":
        return float("-inf")
    try:
        return float(stripped)
    except ValueError:
        return float("nan")


def _js_pow(base: float, exponent: float) -> float:
    """
    Mimics JavaScript's ``**`` operator, returning Infinity on overflow instead
    of raising ``OverflowError``.
    """
    try:
        return float(base) ** float(exponent)
    except OverflowError:
        if base < 0 and exponent != math.floor(exponent):
            return float("nan")
        return float("inf")


def _js_round(value: float) -> float:
    """
    Mimics JavaScript's ``Math.round`` which rounds half away from negative
    infinity (``Math.round(2.5) === 3``, ``Math.round(-2.5) === -2``).
    """
    if math.isnan(value) or math.isinf(value):
        return value
    return math.floor(value + 0.5)
