"""Date validation core module.

Provides the base validation functionality for ``datetime`` instances.
Python's :class:`datetime.datetime` is the natural counterpart of the
TypeScript ``Date`` type.
"""

from collections.abc import Callable
from dataclasses import dataclass
from datetime import datetime


@dataclass
class DateReturnType:
    """
    Result produced by a ``date`` validator.

    Attributes:
        validate: Whether the value passed the check.
        message: Error message describing the failure.
        type: The value that was validated.
    """

    validate: bool
    message: str
    type: object


def date(message: str = "") -> Callable[[object], DateReturnType]:
    """
    Creates a ``datetime`` validator.

    The validator checks that the value is an instance of
    :class:`datetime.datetime`, so values such as plain strings or numbers are
    rejected.

    Args:
        message: Custom error message reported when the value is not a
            ``datetime`` instance (default: empty string).

    Returns:
        A validator function that checks whether the value is a ``datetime``
        instance.

    Example:
        >>> from datetime import datetime
        >>> validator = date()
        >>> validator(datetime(2026, 5, 7)).validate
        True
        >>> validator("2026-05-07").validate
        False
        >>> date("must be a date")("nope").message
        'must be a date'
    """

    def validator(value: object) -> DateReturnType:
        if not isinstance(value, datetime):
            return DateReturnType(validate=False, message=message, type=value)
        return DateReturnType(validate=True, message="", type=value)

    return validator
