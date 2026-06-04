import io
from collections.abc import Callable
from dataclasses import dataclass
from typing import Generic, TypeVar

T = TypeVar("T")


@dataclass(frozen=True)
class FileReturnType(Generic[T]):
    """
    Result produced by a file validator.

    Carries the validated value through the type field so downstream
    composition helpers can keep operating on the original instance.

    Attributes:
        validate: Whether validation succeeded.
        message: The validation message (empty on success).
        type: The value that was checked.
    """

    validate: bool
    message: str
    type: T


def file(
    message: str | None = None,
) -> Callable[[object], FileReturnType[object]]:
    """
    Creates a validator that checks whether a value is a file-like object.

    The TypeScript original accepts ``File`` instances, falling back to ``Blob``
    in runtimes where ``File`` is unavailable. Python has no browser ``File``
    type, so the runtime analog is a binary file-like object (``io.IOBase``)
    or a raw byte buffer (``bytes`` / ``bytearray``).

    Args:
        message: Custom error message for type validation.

    Returns:
        A validator that succeeds for file-like values and fails otherwise.

    Example:
        >>> import io
        >>> validator = file()
        >>> validator(io.BytesIO(b"hello")).validate
        True
        >>> validator(b"hello").validate
        True
        >>> validator("hello").validate
        False
    """

    def validator(value: object) -> FileReturnType[object]:
        if not isinstance(value, (io.IOBase, bytes, bytearray)):
            return FileReturnType(validate=False, message=message or "", type=value)
        return FileReturnType(validate=True, message="", type=value)

    return validator
