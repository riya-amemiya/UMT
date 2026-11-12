from collections.abc import Iterable

from .core import ObjectValidator, object_validator


def omit_keys(
    validator: ObjectValidator,
    keys: Iterable[str],
    message: str = "",
) -> ObjectValidator:
    """
    Removes the given keys from an existing object validator and returns a new
    object validator covering the remaining keys.

    Args:
        validator: Source object validator.
        keys: Keys to drop from the new validator.
        message: Custom error message for the new validator (default: empty
            string).

    Returns:
        A new ``ObjectValidator`` without the omitted keys.

    Example:
        >>> from src.validate.number import number
        >>> from src.validate.string import string
        >>> user = object_validator({"name": string(), "age": number()})
        >>> omitted = omit_keys(user, ["age"])
        >>> omitted({"name": "John"}).validate
        True
        >>> list(omitted.shape)
        ['name']
    """
    omitted_keys = frozenset(keys)
    return object_validator(
        {
            key: property_validator
            for key, property_validator in validator.shape.items()
            if key not in omitted_keys
        },
        message,
    )
