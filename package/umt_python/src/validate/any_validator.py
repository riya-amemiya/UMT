from collections.abc import Callable
from dataclasses import dataclass
from typing import Literal


@dataclass(frozen=True)
class AnyReturnType:
    """
    Result produced by an any validator.

    Exposes the literal "any" tag through the type field so downstream
    composition helpers can map it back to the any runtime type.

    Attributes:
        validate: Whether validation succeeded (always True).
        message: The validation message (always empty).
        type: The literal "any" tag.
    """

    validate: bool
    message: str
    type: Literal["any"]


def any_validator() -> Callable[[object], AnyReturnType]:
    """
    Creates a validator that accepts any value.

    Useful when a position in a schema needs to remain wide open while still
    participating in object, union, and intersection compositional helpers.

    Returns:
        A validator that always succeeds, carrying the "any" type tag.

    Example:
        >>> validator = any_validator()
        >>> validator(123).validate
        True
        >>> validator(None).type
        'any'
    """

    def validator(_value: object) -> AnyReturnType:
        return AnyReturnType(validate=True, message="", type="any")

    return validator
