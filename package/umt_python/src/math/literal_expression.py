import math
import re

from .calculator_core import calculator_core
from .division import division
from .gcd import gcd
from .number_to_js_string import number_to_js_string

_LETTER_RE = re.compile(r"[A-Za-z]+")
_VARIABLE_SPLIT_RE = re.compile(r"([+-]?\d*[A-Za-z]+)|([+-]?\d+)")
_COEFFICIENT_SPLIT_RE = re.compile(r"(\d+)|([A-Za-z]+)")
_FRACTION_OVER_ONE_RE = re.compile(r"(-?)\d+/1")


def literal_expression(x: str) -> str:
    """
    Solves a simple literal equation containing a single-letter variable.

    Args:
        x: Equation string such as ``"x+1=2"`` or ``"2x=6"``.

    Returns:
        The solved value as a string.

    Example:
        >>> literal_expression("x+1=2")
        '1'
        >>> literal_expression("2x=6")
        '3'
        >>> literal_expression("3x+2=8")
        '2'
    """
    sides = x.split("=")
    if len(sides) == 2 and sides[0] == sides[1]:
        return ""

    numerical_part = ""
    variable_part: list[str] = []

    for part in sides:
        if _LETTER_RE.search(part):
            variable_part = _split_filter(_VARIABLE_SPLIT_RE, part)
        else:
            numerical_part = part

    if len(variable_part) > 1 and variable_part[1]:
        inverted_part = (
            variable_part[1]
            .replace("+", "plus")
            .replace("-", "minus")
            .replace("plus", "-")
            .replace("minus", "+")
        )
        variable_part[1] = calculator_core(inverted_part)

    if len(variable_part) > 1 and variable_part[1]:
        sign = "" if variable_part[1].startswith("-") else "+"
        numerical_part = calculator_core(f"{numerical_part}{sign}{variable_part[1]}")
    else:
        numerical_part = calculator_core(numerical_part)

    variable_part = _split_filter(_COEFFICIENT_SPLIT_RE, variable_part[0])

    if math.isnan(_to_number(variable_part[0])):
        return numerical_part

    common_gcd = gcd(_to_number(variable_part[0]), _to_number(numerical_part))
    if common_gcd != 1:
        numerical_part = (
            f"{number_to_js_string(division(_to_number(numerical_part), common_gcd))}"
            f"/{number_to_js_string(division(_to_number(variable_part[0]), common_gcd))}"
        )
        if _FRACTION_OVER_ONE_RE.search(numerical_part):
            return numerical_part.replace("/1", "", 1)
    elif variable_part[0] != "1":
        numerical_part = f"{numerical_part}/{variable_part[0]}"

    return numerical_part


def _split_filter(pattern: re.Pattern[str], value: str) -> list[str]:
    """
    Mimics JavaScript's ``String.prototype.split`` with a capturing regex
    followed by filtering out empty / undefined fragments.
    """
    parts = pattern.split(value)
    return [part for part in parts if part]


def _to_number(value: str) -> float:
    """Mimics JavaScript's ``Number(value)`` coercion, returning NaN on failure."""
    stripped = value.strip()
    if stripped == "":
        return 0.0
    try:
        return float(stripped)
    except ValueError:
        return float("nan")
