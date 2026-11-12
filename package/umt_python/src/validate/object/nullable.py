from collections.abc import Callable
from dataclasses import dataclass
from typing import Literal

from .core import PropertyResult


@dataclass
class NullReturn:
    """
    Result produced by a nullable validator for a ``None`` value.

    Attributes:
        validate: Whether validation succeeded (always True).
        message: The validation message (always empty).
        type: The literal type tag ``"null"``.
    """

    validate: bool
    message: str
    type: Literal["null"] = "null"


def nullable(
    validator: Callable[[object], PropertyResult],
) -> Callable[[object], PropertyResult]:
    """
    Wraps a validator to accept ``None`` values.

    Args:
        validator: Validator function to make nullable.

    Returns:
        A validator that passes for ``None`` or delegates to the wrapped
        validator.

    Example:
        >>> from src.validate.string import string
        >>> validator = nullable(string())
        >>> validator(None).validate
        True
        >>> validator("abc").validate
        True
        >>> validator(1).validate
        False
    """

    def nullable_validator(value: object) -> PropertyResult:
        if value is None:
            return NullReturn(validate=True, message="")
        return validator(value)

    return nullable_validator
