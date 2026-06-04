from collections.abc import Callable
from dataclasses import dataclass
from typing import Literal


@dataclass(frozen=True)
class UnknownReturnType:
    """
    Result produced by an unknown validator.

    Exposes the literal "unknown" tag through the type field so downstream
    composition helpers can map it back to the unknown runtime type, keeping
    callers honest about narrowing before use.

    Attributes:
        validate: Whether validation succeeded (always True).
        message: The validation message (always empty).
        type: The literal "unknown" tag.
    """

    validate: bool
    message: str
    type: Literal["unknown"]


def unknown_validator() -> Callable[[object], UnknownReturnType]:
    """
    Creates a validator that accepts any value but typed as unknown.

    Useful when a position in a schema should accept anything while still
    forcing callers to narrow the value before use, unlike any_validator
    which leaves the value fully open.

    Returns:
        A validator that always succeeds, carrying the "unknown" type tag.

    Example:
        >>> validator = unknown_validator()
        >>> validator(123).validate
        True
        >>> validator(None).type
        'unknown'
    """

    def validator(_value: object) -> UnknownReturnType:
        return UnknownReturnType(validate=True, message="", type="unknown")

    return validator
