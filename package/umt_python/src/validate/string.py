"""String validation core module.

Provides the base validation functionality for string values. JavaScript's
``string`` maps directly onto Python's ``str``.
"""

from collections.abc import Callable, Sequence
from dataclasses import dataclass
from typing import Literal


@dataclass
class StringRule:
    """
    A single validation rule applied to a string value.

    Attributes:
        validate: Predicate that returns True when the value passes the rule.
        message: Error message reported when the predicate fails.
    """

    validate: Callable[[str], bool]
    message: str = ""


@dataclass
class StringReturnType:
    """
    Result produced by a ``string`` validator.

    Attributes:
        validate: Whether the value passed every check.
        message: Error message describing the first failed check.
        type: The literal type tag ``"string"``.
    """

    validate: bool
    message: str
    type: Literal["string"] = "string"


def string(
    option: Sequence[StringRule] | None = None,
    message: str = "",
) -> Callable[[object], StringReturnType]:
    """
    Creates a string validator with optional validation rules.

    Args:
        option: Sequence of validation rules to apply (default: no rules).
        message: Custom error message reported when the value is not a string
            (default: empty string).

    Returns:
        A validator function that checks whether the value is a string and
        applies the configured validation rules.

    Example:
        >>> validator = string()
        >>> validator("abc").validate
        True
        >>> validator(123).validate
        False
        >>> at_most_five = StringRule(
        ...     validate=lambda value: len(value) <= 5,
        ...     message="must be at most 5 characters",
        ... )
        >>> string([at_most_five])("abcdef").message
        'must be at most 5 characters'
    """
    rules = option if option is not None else []

    def validator(value: object) -> StringReturnType:
        if not isinstance(value, str):
            return StringReturnType(validate=False, message=message)
        for rule in rules:
            if not rule.validate(value):
                return StringReturnType(validate=False, message=rule.message)
        return StringReturnType(validate=True, message="")

    return validator
