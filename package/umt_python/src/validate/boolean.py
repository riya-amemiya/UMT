"""Boolean validation core module.

Provides the base validation functionality for boolean values. JavaScript's
``boolean`` maps directly onto Python's ``bool``.
"""

from collections.abc import Callable
from dataclasses import dataclass
from typing import Literal


@dataclass
class BooleanReturnType:
    """
    Result produced by a ``boolean`` validator.

    Attributes:
        validate: Whether the value is a boolean.
        message: Error message describing the failed check.
        type: The literal type tag ``"boolean"``.
    """

    validate: bool
    message: str
    type: Literal["boolean"] = "boolean"


def boolean(message: str = "") -> Callable[[object], BooleanReturnType]:
    """
    Creates a boolean validator.

    Args:
        message: Custom error message reported when the value is not a boolean
            (default: empty string).

    Returns:
        A validator function that checks whether the value is a boolean.

    Example:
        >>> validator = boolean()
        >>> validator(True).validate
        True
        >>> validator(1).validate
        False
        >>> boolean("must be a boolean")("true").message
        'must be a boolean'
    """

    def validator(value: object) -> BooleanReturnType:
        if not isinstance(value, bool):
            return BooleanReturnType(validate=False, message=message)
        return BooleanReturnType(validate=True, message="")

    return validator
