from collections.abc import Callable
from dataclasses import dataclass
from typing import Literal


@dataclass(frozen=True)
class NeverReturnType:
    """
    Result produced by a never validator.

    Exposes the literal "never" tag through the type field so downstream
    composition helpers can map it back to the never runtime type, useful
    for marking positions in a schema that must never be filled.

    Attributes:
        validate: Whether validation succeeded (always False).
        message: The configured validation message.
        type: The literal "never" tag.
    """

    validate: bool
    message: str
    type: Literal["never"]


def never_validator(message: str = "") -> Callable[[object], NeverReturnType]:
    """
    Creates a validator that always fails.

    Useful for marking positions in a schema that must never be filled, for
    example exhaustive union members that should be unreachable.

    Args:
        message: Custom error message for the validation failure.

    Returns:
        A validator that always fails, carrying the "never" type tag.

    Example:
        >>> validator = never_validator()
        >>> validator(123).validate
        False
        >>> validator(None).type
        'never'
    """

    def validator(_value: object) -> NeverReturnType:
        return NeverReturnType(validate=False, message=message, type="never")

    return validator
