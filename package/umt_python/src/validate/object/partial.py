from .core import ObjectValidator, object_validator
from .optional import OptionalValidator, optional


def partial(validator: ObjectValidator, message: str = "") -> ObjectValidator:
    """
    Wraps every property validator of ``validator`` with ``optional``.

    Validators that are already optional are kept as-is so the wrapper stays
    idempotent.

    Args:
        validator: Source object validator.
        message: Custom error message for the new validator (default: empty
            string).

    Returns:
        A new ``ObjectValidator`` where every key accepts ``None``.

    Example:
        >>> from src.validate.string import string
        >>> user = object_validator({"name": string()})
        >>> user({}).validate
        False
        >>> partial(user)({}).validate
        True
        >>> partial(user)({"name": 1}).validate
        False
    """
    return object_validator(
        {
            key: current
            if isinstance(current, OptionalValidator)
            else optional(current)
            for key, current in validator.shape.items()
        },
        message,
    )
