"""String validation module for literal union (allowed values) check.

Provides validation functionality for checking if a string is one of the
allowed literal values, useful for validating string literal union types.
"""

from collections.abc import Callable, Iterable
from dataclasses import dataclass


@dataclass
class OneOfReturnType:
    """
    Result produced by a ``one_of`` validator.

    Attributes:
        validate: Whether the value is one of the allowed values.
        message: Error message reported on failure, empty on success.
        type: The value that was validated.
    """

    validate: bool
    message: str
    type: object


def one_of(
    values: Iterable[str],
    message: str = "",
) -> Callable[[object], OneOfReturnType]:
    """
    Creates a top-level validator that checks if a string value is one of the
    given allowed literal values.

    Args:
        values: The allowed string values.
        message: Custom error message for validation failure (default: empty
            string).

    Returns:
        A validator function that succeeds only for the allowed values.

    Example:
        >>> validator = one_of(["standard", "squat"])
        >>> validator("standard").validate
        True
        >>> validator("round").validate
        False
        >>> one_of(["a"], "must be a")("b").message
        'must be a'
    """
    allowed = frozenset(values)

    def validator(value: object) -> OneOfReturnType:
        is_valid = isinstance(value, str) and value in allowed
        return OneOfReturnType(
            validate=is_valid,
            message="" if is_valid else message,
            type=value,
        )

    return validator
