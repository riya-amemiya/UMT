from collections.abc import Callable
from dataclasses import dataclass

from .core import PropertyResult


@dataclass
class IntersectionReturnType:
    """
    Result produced by an ``intersection`` validator.

    Attributes:
        validate: Whether every member validator accepted the value.
        message: Message of the first failing member, empty on success.
        type: The original value passed to the validator.
    """

    validate: bool
    message: str
    type: object


def intersection(
    *validators: Callable[[object], PropertyResult],
) -> Callable[[object], IntersectionReturnType]:
    """
    Creates an intersection validator that passes only if all given validators
    pass.

    Args:
        *validators: Validator functions to compose as an intersection
            (logical AND).

    Returns:
        A validator that checks if the value matches all validators. On
        failure it carries the message of the first failing validator.

    Example:
        >>> from src.validate.object.core import object_validator
        >>> from src.validate.number import number
        >>> from src.validate.string import string
        >>> validator = intersection(
        ...     object_validator({"name": string()}),
        ...     object_validator({"age": number()}),
        ... )
        >>> validator({"name": "John", "age": 30}).validate
        True
        >>> validator({"name": "John"}).validate
        False
    """

    def intersection_validator(value: object) -> IntersectionReturnType:
        for validator in validators:
            result = validator(value)
            if not result.validate:
                return IntersectionReturnType(
                    validate=False, message=result.message, type=value
                )
        return IntersectionReturnType(validate=True, message="", type=value)

    return intersection_validator
