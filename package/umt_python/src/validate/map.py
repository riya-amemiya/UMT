"""Map validation core module.

Provides validation for mapping instances. The validator can optionally
delegate to per-entry validators for keys and values, mirroring how
``array_of`` validates each element of a list. Python's :class:`dict` is the
natural counterpart of the TypeScript ``Map`` type.
"""

from collections.abc import Callable, Mapping
from dataclasses import dataclass
from typing import Protocol


class EntryResult(Protocol):
    """
    Minimal shape returned by an entry validator.

    Only the ``validate`` and ``message`` fields are read while iterating over
    the map entries, so any validator return type exposing them is accepted.

    Attributes:
        validate: Whether the key or value passed validation.
        message: Error message describing why the entry failed.
    """

    validate: bool
    message: str


@dataclass
class MapReturnType:
    """
    Result produced by a ``map_validator``.

    The ``type`` field preserves the original mapping so downstream composition
    helpers can flow the validated value through unchanged.

    Attributes:
        validate: Whether the value is a mapping and every entry passed.
        message: Error message describing the first failure, empty on success.
        type: The original value passed to the validator.
    """

    validate: bool
    message: str
    type: object


def map_validator(
    key_validator: Callable[[object], EntryResult] | None = None,
    value_validator: Callable[[object], EntryResult] | None = None,
    message: str = "",
) -> Callable[[object], MapReturnType]:
    """
    Creates a map validator.

    The validator can optionally accept per-entry key and value validators that
    are applied to every entry of the mapping. When provided, the entry
    validators short-circuit on the first failure and surface the failing
    message, mirroring ``array_of``.

    Args:
        key_validator: Validator applied to every key (default: ``None``).
        value_validator: Validator applied to every value (default: ``None``).
        message: Custom error message reported when the value is not a mapping
            (default: empty string).

    Returns:
        A validator function for mappings whose entries satisfy the given
        validators. It stops at the first failing entry and propagates that
        entry's message.

    Example:
        >>> def string_validator(value: object) -> MapReturnType:
        ...     ok = isinstance(value, str)
        ...     return MapReturnType(
        ...         validate=ok,
        ...         message="" if ok else "not a string",
        ...         type=value,
        ...     )
        >>> def number_validator(value: object) -> MapReturnType:
        ...     ok = isinstance(value, (int, float)) and not isinstance(
        ...         value, bool
        ...     )
        ...     return MapReturnType(
        ...         validate=ok,
        ...         message="" if ok else "not a number",
        ...         type=value,
        ...     )
        >>> validator = map_validator(string_validator, number_validator)
        >>> validator({"a": 1, "b": 2}).validate
        True
        >>> validator({"a": "x"}).validate
        False
        >>> validator("not a map").validate
        False
    """

    def validator(value: object) -> MapReturnType:
        if not isinstance(value, Mapping):
            return MapReturnType(validate=False, message=message, type=value)
        for entry_key, entry_value in value.items():
            if key_validator is not None:
                key_result = key_validator(entry_key)
                if not key_result.validate:
                    return MapReturnType(
                        validate=False,
                        message=key_result.message,
                        type=value,
                    )
            if value_validator is not None:
                value_result = value_validator(entry_value)
                if not value_result.validate:
                    return MapReturnType(
                        validate=False,
                        message=value_result.message,
                        type=value,
                    )
        return MapReturnType(validate=True, message="", type=value)

    return validator
