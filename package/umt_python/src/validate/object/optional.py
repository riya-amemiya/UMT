from collections.abc import Callable
from dataclasses import dataclass
from typing import Literal

from .core import PropertyResult


@dataclass
class UndefinedReturn:
    """
    Result produced by an optional validator for a missing value.

    Attributes:
        validate: Whether validation succeeded (always True).
        message: The validation message (always empty).
        type: The literal type tag ``"undefined"``.
    """

    validate: bool
    message: str
    type: Literal["undefined"] = "undefined"


class OptionalValidator:
    """
    Optional validator augmented with a reference to the wrapped validator.

    The ``inner`` and ``is_optional`` attributes are used by ``required`` to
    unwrap optional layers when rebuilding a shape. Python's ``None`` is the
    counterpart of the TypeScript ``undefined`` value.

    Attributes:
        inner: The wrapped validator.
        is_optional: Marker that identifies optional validators (always True).
    """

    is_optional = True

    def __init__(self, inner: Callable[[object], PropertyResult]) -> None:
        self.inner = inner

    def __call__(self, value: object = None) -> PropertyResult:
        if value is None:
            return UndefinedReturn(validate=True, message="")
        return self.inner(value)


def optional(validator: Callable[[object], PropertyResult]) -> OptionalValidator:
    """
    Wraps a validator to accept missing (``None``) values.

    Args:
        validator: Validator function to make optional.

    Returns:
        A validator that passes for ``None`` or delegates to the wrapped
        validator.

    Example:
        >>> from src.validate.string import string
        >>> validator = optional(string())
        >>> validator(None).validate
        True
        >>> validator("abc").validate
        True
        >>> validator(1).validate
        False
    """
    return OptionalValidator(validator)
