"""Record validation core module.

Provides validation for dictionary objects with dynamic keys, applying one
validator to every key and another to every value. It is the Python
counterpart of the TypeScript ``record`` factory.
"""

from collections.abc import Callable
from dataclasses import dataclass
from typing import Protocol

from .is_dictionary_object import is_dictionary_object


class EntryResult(Protocol):
    """
    Minimal shape returned by a key or value validator.

    Only the ``validate`` and ``message`` fields are read while iterating over
    the entries, so any validator return type exposing them is accepted.

    Attributes:
        validate: Whether the key or value passed validation.
        message: Error message describing why the entry failed.
    """

    validate: bool
    message: str


@dataclass
class RecordReturnType:
    """
    Result produced by a ``record`` validator.

    The ``type`` field preserves the original value so downstream composition
    helpers can flow the validated value through unchanged.

    Attributes:
        validate: Whether the value is a dictionary and every entry passed.
        message: Error message describing the first failure, empty on success.
        type: The original value passed to the validator.
    """

    validate: bool
    message: str
    type: object


def record(
    key_validator: Callable[[object], EntryResult],
    value_validator: Callable[[object], EntryResult],
    message: str = "",
) -> Callable[[object], RecordReturnType]:
    """
    Creates a record validator for dictionaries with dynamic keys.

    Every key is validated with ``key_validator`` and then its value with
    ``value_validator``; iteration short-circuits at the first failure and
    surfaces the failing message.

    Args:
        key_validator: Validator applied to every key.
        value_validator: Validator applied to every value.
        message: Custom error message reported when the value is not a
            dictionary (default: empty string).

    Returns:
        A validator function for dictionaries whose entries satisfy both
        validators.

    Example:
        >>> from src.validate.number import number
        >>> from src.validate.string import string
        >>> validator = record(string(), number())
        >>> validator({"a": 1, "b": 2}).validate
        True
        >>> validator({"a": "x"}).validate
        False
        >>> validator([1, 2]).validate
        False
    """

    def validator(value: object) -> RecordReturnType:
        if not is_dictionary_object(value) or not isinstance(value, dict):
            return RecordReturnType(validate=False, message=message, type=value)
        for entry_key, entry_value in value.items():
            key_result = key_validator(entry_key)
            if not key_result.validate:
                return RecordReturnType(
                    validate=False, message=key_result.message, type=value
                )
            value_result = value_validator(entry_value)
            if not value_result.validate:
                return RecordReturnType(
                    validate=False, message=value_result.message, type=value
                )
        return RecordReturnType(validate=True, message="", type=value)

    return validator
