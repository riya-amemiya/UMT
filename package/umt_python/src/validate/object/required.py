from .core import ObjectValidator, object_validator
from .optional import OptionalValidator


def required(validator: ObjectValidator, message: str = "") -> ObjectValidator:
    """
    Strips ``optional`` wrappers from every property validator of
    ``validator``.

    Validators that are not wrapped in ``optional`` are kept as-is.

    Args:
        validator: Source object validator.
        message: Custom error message for the new validator (default: empty
            string).

    Returns:
        A new ``ObjectValidator`` where every optional layer is removed.

    Example:
        >>> from src.validate.object.optional import optional
        >>> from src.validate.string import string
        >>> user = object_validator({"name": optional(string())})
        >>> user({}).validate
        True
        >>> required(user)({}).validate
        False
        >>> required(user)({"name": "John"}).validate
        True
    """
    return object_validator(
        {
            key: current.inner if isinstance(current, OptionalValidator) else current
            for key, current in validator.shape.items()
        },
        message,
    )
