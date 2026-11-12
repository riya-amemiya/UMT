from collections.abc import Callable
from dataclasses import dataclass

from .core import PropertyResult


@dataclass
class UnionReturnType:
    """
    Result produced by a ``union`` validator.

    Attributes:
        validate: Whether any member validator accepted the value.
        message: Message of the last failing member, empty on success.
        type: The original value passed to the validator.
    """

    validate: bool
    message: str
    type: object


def union(
    *validators: Callable[[object], PropertyResult],
) -> Callable[[object], UnionReturnType]:
    """
    Creates a union validator that passes if any of the given validators pass.

    Args:
        *validators: Validator functions to compose as a union (logical OR).

    Returns:
        A validator that checks if the value matches any of the validators.
        On failure it carries the message of the last validator.

    Example:
        >>> from src.validate.number import number
        >>> from src.validate.string import string
        >>> validator = union(string(), number())
        >>> validator("abc").validate
        True
        >>> validator(1).validate
        True
        >>> validator(None).validate
        False
    """

    def union_validator(value: object) -> UnionReturnType:
        last_message = ""
        for validator in validators:
            result = validator(value)
            if result.validate:
                return UnionReturnType(validate=True, message="", type=value)
            last_message = result.message
        return UnionReturnType(validate=False, message=last_message, type=value)

    return union_validator
