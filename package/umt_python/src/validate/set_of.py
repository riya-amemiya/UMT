"""Set validation core module.

Provides a higher-order validator that applies a single element validator to
every item of a set. It is the Python counterpart of the TypeScript ``setOf``
factory, which itself mirrors ``arrayOf`` for consistency.

The function is named ``set_of`` to avoid colliding with the built-in ``set``
type, matching the TypeScript naming where ``setOf`` avoids clashing with the
``Object`` module's ``set`` helper.
"""

from collections.abc import Callable
from dataclasses import dataclass
from typing import Protocol


class ElementResult(Protocol):
    """
    Minimal shape returned by an element validator.

    Only the ``validate`` and ``message`` fields are read while iterating over
    the set, so any validator return type exposing them is accepted.

    Attributes:
        validate: Whether the element passed validation.
        message: Error message describing why the element failed.
    """

    validate: bool
    message: str


@dataclass
class SetOfReturnType:
    """
    Result produced by a ``set_of`` validator.

    The ``type`` field preserves the original set so downstream composition
    helpers can flow the validated value through unchanged.

    Attributes:
        validate: Whether every element passed validation.
        message: Error message describing the first failure, empty on success.
        type: The original value passed to the validator.
    """

    validate: bool
    message: str
    type: object


def set_of(
    validator: Callable[[object], ElementResult] | None = None,
    message: str = "",
) -> Callable[[object], SetOfReturnType]:
    """
    Creates a set validator. When a per-element validator is supplied, every
    element of the set must satisfy it; iteration short-circuits at the first
    failure and surfaces the failing message.

    Args:
        validator: Validator applied to every element (for example a string
            validator). When omitted, only the set type itself is checked
            (default: ``None``).
        message: Custom error message reported when the value is not a set
            (default: empty string).

    Returns:
        A validator function for sets whose elements satisfy the given
        validator. It stops at the first failing element and propagates that
        element's message.

    Example:
        >>> def string_validator(value: object) -> SetOfReturnType:
        ...     ok = isinstance(value, str)
        ...     return SetOfReturnType(
        ...         validate=ok,
        ...         message="" if ok else "not a string",
        ...         type=value,
        ...     )
        >>> set_of(string_validator)({"a", "b"}).validate
        True
        >>> set_of(string_validator)({1}).validate
        False
        >>> set_of()([1, 2]).validate
        False
        >>> set_of()(set()).validate
        True
    """

    def set_validator(value: object) -> SetOfReturnType:
        if not isinstance(value, set):
            return SetOfReturnType(validate=False, message=message, type=value)
        if validator is not None:
            for item in value:
                result = validator(item)
                if not result.validate:
                    return SetOfReturnType(
                        validate=False, message=result.message, type=value
                    )
        return SetOfReturnType(validate=True, message="", type=value)

    return set_validator
