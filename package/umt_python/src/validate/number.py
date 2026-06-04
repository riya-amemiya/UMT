"""Number validation core module.

Provides the base validation functionality for number values. JavaScript's
``number`` covers both integers and floating point values, so its natural
Python counterpart is ``int`` or ``float`` (excluding ``bool``).
"""

from collections.abc import Callable, Sequence
from dataclasses import dataclass
from typing import Literal


@dataclass
class NumberRule:
    """
    A single validation rule applied to a number value.

    Attributes:
        validate: Predicate that returns True when the value passes the rule.
        message: Error message reported when the predicate fails.
    """

    validate: Callable[[float], bool]
    message: str = ""


@dataclass
class NumberReturnType:
    """
    Result produced by a ``number`` validator.

    Attributes:
        validate: Whether the value passed every check.
        message: Error message describing the first failed check.
        type: The literal type tag ``"number"``.
    """

    validate: bool
    message: str
    type: Literal["number"] = "number"


def number(
    option: Sequence[NumberRule] | None = None,
    message: str = "",
) -> Callable[[object], NumberReturnType]:
    """
    Creates a number validator with optional validation rules.

    Args:
        option: Sequence of validation rules to apply (default: no rules).
        message: Custom error message reported when the value is not a number
            (default: empty string).

    Returns:
        A validator function that checks whether the value is a number and
        applies the configured validation rules.

    Example:
        >>> validator = number()
        >>> validator(5).validate
        True
        >>> validator("5").validate
        False
        >>> at_most_ten = NumberRule(
        ...     validate=lambda value: value <= 10,
        ...     message="must be at most 10",
        ... )
        >>> number([at_most_ten])(11).message
        'must be at most 10'
    """
    rules = option if option is not None else []

    def validator(value: object) -> NumberReturnType:
        if not isinstance(value, (int, float)) or isinstance(value, bool):
            return NumberReturnType(validate=False, message=message)
        for rule in rules:
            if not rule.validate(value):
                return NumberReturnType(validate=False, message=rule.message)
        return NumberReturnType(validate=True, message="")

    return validator
