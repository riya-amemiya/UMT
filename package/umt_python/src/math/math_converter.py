import re

from .math_separator import math_separator


def math_converter(equation: str) -> str:
    """
    Expands square of n into a sum of simpler multiplications.

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
        match = re.search(r"\d+\.?(\d+)?(\*|\^)\d+\.?(\d+)?", converted_equation)

        if not match:
            return converted_equation

        matched_string = match.group()
        parts = re.split(r"(\*|\^)", matched_string)
        operand1 = parts[0]
        operator = parts[1]
        operand2 = parts[2] if len(parts) > 2 else None

        if operand1 == operand2 or (operand2 and operator == "^"):
            primary, remainder = math_separator(float(operand1))

            if not primary:
                return converted_equation

            converted_equation = f"{float(operand1) + remainder}*{int(primary)}+"

            if remainder <= 100:
                converted_equation += f"{int(remainder)}*{int(remainder)}"
            else:
                converted_equation += math_converter(
                    f"{int(remainder)}*{int(remainder)}"
                )
        else:
            return converted_equation
