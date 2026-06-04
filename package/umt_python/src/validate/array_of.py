"""Array validation core module.

Provides a higher-order validator that applies a single element validator to
every item of a list. It is the Python counterpart of the TypeScript
``arrayOf`` factory.
"""

from collections.abc import Callable
from dataclasses import dataclass
from typing import Protocol


class ElementResult(Protocol):
    """
    Minimal shape returned by an element validator.

    Only the ``validate`` and ``message`` fields are read while iterating over
    the array, so any validator return type exposing them is accepted.

    Attributes:
        validate: Whether the element passed validation.
        message: Error message describing why the element failed.
    """

    validate: bool
    message: str


@dataclass
class ArrayOfReturnType:
    """
    Result produced by an ``array_of`` validator.

    The ``type`` field preserves the original list so downstream composition
    helpers can flow the validated value through unchanged.

    Attributes:
        validate: Whether every element passed validation.
        message: Error message describing the first failure, empty on success.
        type: The original value passed to the validator.
    """

    validate: bool
    message: str
    type: object


def array_of(
    validator: Callable[[object], ElementResult],
    message: str = "",
) -> Callable[[object], ArrayOfReturnType]:
    """
    Creates an array validator that validates every element with a single
    validator.

    Args:
        validator: Validator applied to each element (for example an object
            validator).
        message: Custom error message reported when the value is not a list
            (default: empty string).

    Returns:
        A validator function for lists whose elements satisfy the given
        validator. It stops at the first failing element and propagates that
        element's message.

    Example:
        >>> from src.validate.is_string import is_string
        >>> def string_validator(value: object) -> ArrayOfReturnType:
        ...     ok = is_string(value)
        ...     return ArrayOfReturnType(
        ...         validate=ok,
        ...         message="" if ok else "not a string",
        ...         type=value,
        ...     )
        >>> array_of(string_validator)(["a", "b"]).validate
        True
        >>> array_of(string_validator)(["a", 1]).validate
        False
        >>> array_of(string_validator)("not a list").validate
        False
    """

    def array_validator(values: object) -> ArrayOfReturnType:
        if not isinstance(values, list):
            return ArrayOfReturnType(validate=False, message=message, type=values)
        for value in values:
            result = validator(value)
            if not result.validate:
                return ArrayOfReturnType(
                    validate=False, message=result.message, type=values
                )
        return ArrayOfReturnType(validate=True, message="", type=values)

    return array_validator
