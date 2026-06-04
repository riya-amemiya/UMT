"""Object validation core module.

Provides validation functionality for objects (Python dicts) with
type-specific validation rules for each property. It is the Python counterpart
of the TypeScript ``object`` factory.
"""

from collections.abc import Callable, Mapping
from dataclasses import dataclass
from typing import Protocol

from src.validate.is_dictionary_object import is_dictionary_object


class PropertyResult(Protocol):
    """
    Minimal shape returned by a property validator.

    Only the ``validate`` and ``message`` fields are read while iterating over
    the shape, so any validator return type exposing them is accepted.

    Attributes:
        validate: Whether the property passed validation.
        message: Error message describing why the property failed.
    """

    validate: bool
    message: str


ObjectShape = Mapping[str, Callable[[object], PropertyResult]]


@dataclass
class ObjectReturnType:
    """
    Result produced by an ``object_validator`` validator.

    The ``type`` field preserves the original value so downstream composition
    helpers can flow the validated value through unchanged.

    Attributes:
        validate: Whether every property passed validation.
        message: Error message describing the first failure, empty on success.
        type: The original value passed to the validator.
    """

    validate: bool
    message: str
    type: object


class ObjectValidator:
    """
    Callable object validator augmented with the original ``shape`` map.

    The shape map is exposed via the ``shape`` attribute so derived helpers
    such as ``pick``, ``omit``, ``partial``, and ``required`` can compose new
    object validators from existing ones.

    Attributes:
        shape: The per-property validator map used to build this validator.
    """

    def __init__(self, shape: ObjectShape, message: str = "") -> None:
        self.shape = shape
        self._message = message

    def __call__(self, value: object) -> ObjectReturnType:
        if not is_dictionary_object(value) or not isinstance(value, dict):
            return ObjectReturnType(validate=False, message=self._message, type=value)
        for key, validator in self.shape.items():
            if not validator(value.get(key)).validate:
                return ObjectReturnType(
                    validate=False,
                    message=validator(value).message,
                    type=value,
                )
        return ObjectReturnType(validate=True, message="", type=value)


def object_validator(
    option: ObjectShape | None = None,
    message: str = "",
) -> ObjectValidator:
    """
    Creates an object validator with property-specific validation rules.

    Args:
        option: Mapping of property names to validation functions. When omitted,
            an empty shape is used and every dictionary value validates.
        message: Custom error message reported when the value is not a
            dictionary object (default: empty string).

    Returns:
        An ``ObjectValidator`` whose ``shape`` attribute exposes the original
        ``option`` map so it can compose with ``pick``, ``omit``, ``partial``,
        and ``required``. It stops at the first failing property and propagates
        that property's message.

    Example:
        >>> from src.validate.is_string import is_string
        >>> def name_validator(value: object) -> ObjectReturnType:
        ...     ok = is_string(value)
        ...     return ObjectReturnType(
        ...         validate=ok,
        ...         message="" if ok else "not a string",
        ...         type=value,
        ...     )
        >>> validator = object_validator({"name": name_validator})
        >>> validator({"name": "John"}).validate
        True
        >>> validator({"name": 42}).validate
        False
        >>> validator("not a dict").validate
        False
        >>> object_validator()({"anything": 1}).validate
        True
    """
    return ObjectValidator(option if option is not None else {}, message)
