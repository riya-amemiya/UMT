"""Tuple validation core module.

Provides a higher-order validator for fixed-length heterogeneous sequences
whose elements are validated position by position. It is the Python
counterpart of the TypeScript ``tuple`` factory.

The function is named ``tuple_validator`` to avoid shadowing the built-in
``tuple`` type.
"""

from collections.abc import Callable, Sequence
from dataclasses import dataclass
from typing import Protocol


class ElementResult(Protocol):
    """
    Minimal shape returned by an element validator.

    Only the ``validate`` and ``message`` fields are read while iterating over
    the sequence, so any validator return type exposing them is accepted.

    Attributes:
        validate: Whether the element passed validation.
        message: Error message describing why the element failed.
    """

    validate: bool
    message: str


@dataclass
class TupleReturnType:
    """
    Result produced by a ``tuple_validator``.

    The ``type`` field preserves the original value so downstream composition
    helpers can flow the validated value through unchanged.

    Attributes:
        validate: Whether the length matched and every element passed.
        message: Error message describing the first failure, empty on success.
        type: The original value passed to the validator.
    """

    validate: bool
    message: str
    type: object


def tuple_validator(
    validators: Sequence[Callable[[object], ElementResult]],
    message: str = "",
) -> Callable[[object], TupleReturnType]:
    """
    Creates a tuple validator for fixed-length sequences whose elements are
    validated with the validator at the same index.

    Both ``list`` (the counterpart of a JavaScript array) and ``tuple`` values
    are accepted.

    Args:
        validators: Validators applied to the element at the same index.
        message: Custom error message reported when the value is not a list or
            tuple, or when its length differs from the number of validators
            (default: empty string).

    Returns:
        A validator function for sequences of exactly ``len(validators)``
        elements. It stops at the first failing element and propagates that
        element's message.

    Example:
        >>> from src.validate.number import number
        >>> from src.validate.string import string
        >>> validator = tuple_validator([string(), number()])
        >>> validator(["a", 1]).validate
        True
        >>> validator(["a", "b"]).validate
        False
        >>> validator(["a"]).validate
        False
    """
    entries = tuple(validators)
    length = len(entries)

    def validator(value: object) -> TupleReturnType:
        if not isinstance(value, (list, tuple)) or len(value) != length:
            return TupleReturnType(validate=False, message=message, type=value)
        for element_validator, element in zip(entries, value, strict=True):
            result = element_validator(element)
            if not result.validate:
                return TupleReturnType(
                    validate=False, message=result.message, type=value
                )
        return TupleReturnType(validate=True, message="", type=value)

    return validator
