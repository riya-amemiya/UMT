"""BigInt validation core module.

Provides the base validation functionality for big integer values. Python's
``int`` is arbitrary precision, so it is the natural counterpart of the
TypeScript ``bigint`` type.
"""

from collections.abc import Callable, Sequence
from dataclasses import dataclass
from typing import Literal


@dataclass
class BigIntRule:
    """
    A single validation rule applied to a big integer value.

    Attributes:
        validate: Predicate that returns True when the value passes the rule.
        message: Error message reported when the predicate fails.
    """

    validate: Callable[[int], bool]
    message: str = ""


@dataclass
class BigIntReturnType:
    """
    Result produced by a ``bigint`` validator.

    Attributes:
        validate: Whether the value passed every check.
        message: Error message describing the first failed check.
        type: The literal type tag ``"bigint"``.
    """

    validate: bool
    message: str
    type: Literal["bigint"] = "bigint"


def bigint(
    option: Sequence[BigIntRule] | None = None,
    message: str = "",
) -> Callable[[object], BigIntReturnType]:
    """
    Creates a big integer validator with optional validation rules.

    Args:
        option: Sequence of validation rules to apply (default: no rules).
        message: Custom error message reported when the value is not a big
            integer (default: empty string).

    Returns:
        A validator function that checks whether the value is a big integer
        and applies the configured validation rules.

    Example:
        >>> validator = bigint()
        >>> validator(123).validate
        True
        >>> validator("123").validate
        False
        >>> positive = BigIntRule(
        ...     validate=lambda value: value > 0,
        ...     message="must be positive",
        ... )
        >>> bigint([positive])(-1).message
        'must be positive'
    """
    rules = option if option is not None else []

    def validator(value: object) -> BigIntReturnType:
        if not isinstance(value, int) or isinstance(value, bool):
            return BigIntReturnType(validate=False, message=message)
        for rule in rules:
            if not rule.validate(value):
                return BigIntReturnType(validate=False, message=rule.message)
        return BigIntReturnType(validate=True, message="")

    return validator
