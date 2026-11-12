"""Literal validation core module.

Provides validation for a single primitive literal value compared with
JavaScript strict equality (``===``) semantics. It is the Python counterpart
of the TypeScript ``literal`` factory.
"""

from collections.abc import Callable
from dataclasses import dataclass


@dataclass
class LiteralReturnType:
    """
    Result produced by a ``literal`` validator.

    Attributes:
        validate: Whether the value is strictly equal to the literal.
        message: Error message reported on failure, empty on success.
        type: The value that was validated.
    """

    validate: bool
    message: str
    type: object


def _is_number(value: object) -> bool:
    return isinstance(value, (int, float)) and not isinstance(value, bool)


def _strict_equal(value: object, expected: object) -> bool:
    if _is_number(value) and _is_number(expected):
        return value == expected
    return type(value) is type(expected) and value == expected


def literal(
    expected: str | int | float | bool | None,
    message: str = "",
) -> Callable[[object], LiteralReturnType]:
    """
    Creates a validator that accepts exactly one primitive value.

    Values are compared like JavaScript ``===``: numbers (``int`` and
    ``float``, excluding ``bool``) compare by value, so ``literal(1)`` accepts
    ``1.0`` and ``literal(0)`` accepts ``-0.0`` while ``literal(float("nan"))``
    never matches. Other values must have the same type and compare equal, so
    ``literal(True)`` rejects ``1`` and ``literal("1")`` rejects ``1``.
    ``None`` stands for both JavaScript ``null`` and ``undefined``.

    Args:
        expected: The only value accepted by the validator.
        message: Custom error message for validation failure (default: empty
            string).

    Returns:
        A validator function that succeeds only for the literal value.

    Example:
        >>> validator = literal("admin")
        >>> validator("admin").validate
        True
        >>> validator("user").validate
        False
        >>> literal(True)(1).validate
        False
        >>> literal(1, "must be 1")(2).message
        'must be 1'
    """

    def validator(value: object) -> LiteralReturnType:
        is_valid = _strict_equal(value, expected)
        return LiteralReturnType(
            validate=is_valid,
            message="" if is_valid else message,
            type=value,
        )

    return validator
