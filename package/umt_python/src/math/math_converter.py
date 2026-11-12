import re

from .js_string_to_number import js_string_to_number
from .math_separator import math_separator
from .number_to_js_string import number_to_js_string

_MUL_OR_EXP_RE = re.compile(r"\d+\.?(\d+)?(\*|\^)\d+\.?(\d+)?")
_OP_SPLIT_RE = re.compile(r"(\*|\^)")


def math_converter(equation: str) -> str:
    """
    Expands square of n into a sum of simpler multiplications.

    An expansion that reproduces its own input, such as ``5*5``, is returned
    after the first repetition instead of being expanded forever.

    Args:
        equation: Mathematical expression to convert.

    Returns:
        Converted expression.

    Example:
        >>> math_converter("1250*1250")
        '1500*1000+400*100+200*100+50*50'
    """
    converted_equation = equation

    while True:
        match = _MUL_OR_EXP_RE.search(converted_equation)

        if not match:
            return converted_equation

        operand1, operator, operand2 = _OP_SPLIT_RE.split(match.group())[:3]

        if operand1 != operand2 and operator != "^":
            return converted_equation

        primary, remainder = math_separator(operand1)

        if not primary:
            return converted_equation

        remainder_string = number_to_js_string(remainder)
        square = f"{remainder_string}*{remainder_string}"
        expanded = (
            f"{number_to_js_string(js_string_to_number(operand1) + remainder)}"
            f"*{number_to_js_string(primary)}+"
            f"{square if remainder <= 100 else math_converter(square)}"
        )

        if expanded == converted_equation:
            return expanded

        converted_equation = expanded
