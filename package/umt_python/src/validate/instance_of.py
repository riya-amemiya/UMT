from collections.abc import Callable
from dataclasses import dataclass
from typing import Generic, TypeVar

T = TypeVar("T")


@dataclass(frozen=True)
class InstanceOfReturnType(Generic[T]):
    """
    Result produced by an instance_of validator.

    Carries the inspected value through the type field so downstream
    composition helpers can keep working with the original instance.

    Attributes:
        validate: Whether the value is an instance of the constructor.
        message: The validation message (empty on success).
        type: The inspected value.
    """

    validate: bool
    message: str
    type: T


def instance_of(
    class_constructor: type[T],
    message: str = "",
) -> Callable[[T], InstanceOfReturnType[T]]:
    """
    Creates a validator that checks whether a value is an instance of the
    given constructor.

    Subclasses of the constructor satisfy the validator, matching native
    isinstance semantics. The factory is named instance_of because instance
    relationships are checked, mirroring the TypeScript instanceOf helper.

    Args:
        class_constructor: Constructor whose instances are accepted.
        message: Custom error message for validation failure (default: "").

    Returns:
        A validator function for instances of the constructor.

    Example:
        >>> validator = instance_of(int)
        >>> validator(123).validate
        True
        >>> validator("123").validate
        False
        >>> instance_of(int, "must be an int")("x").message
        'must be an int'
    """

    def validator(value: T) -> InstanceOfReturnType[T]:
        if not isinstance(value, class_constructor):
            return InstanceOfReturnType(
                validate=False,
                message=message,
                type=value,
            )
        return InstanceOfReturnType(validate=True, message="", type=value)

    return validator
