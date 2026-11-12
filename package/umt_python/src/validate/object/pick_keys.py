from collections.abc import Iterable

from .core import ObjectValidator, object_validator


def pick_keys(
    validator: ObjectValidator,
    keys: Iterable[str],
    message: str = "",
) -> ObjectValidator:
    """
    Picks the given keys from an existing object validator and returns a new
    object validator covering only those keys.

    Args:
        validator: Source object validator.
        keys: Keys to retain on the new validator.
        message: Custom error message for the picked validator (default: empty
            string).

    Returns:
        A new ``ObjectValidator`` scoped to the picked keys.

    Example:
        >>> from src.validate.number import number
        >>> from src.validate.string import string
        >>> user = object_validator({"name": string(), "age": number()})
        >>> picked = pick_keys(user, ["name"])
        >>> picked({"name": "John"}).validate
        True
        >>> list(picked.shape)
        ['name']
    """
    source_shape = validator.shape
    return object_validator({key: source_shape[key] for key in keys}, message)
