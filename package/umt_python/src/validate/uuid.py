import re
from collections.abc import Sequence

from .string import StringRule

_VALID_UUID_VERSIONS = frozenset({1, 2, 3, 4, 5, 6, 7})


def uuid(
    versions: Sequence[int] | None = None,
    message: str = "",
) -> StringRule:
    """
    Creates a rule for checking if a string is a valid UUID.

    Versions outside ``1`` to ``7`` are ignored. When no supported version
    remains the rule rejects every value.

    Args:
        versions: Supported UUID versions (default: ``[4]``).
        message: Custom error message for validation failure (default: empty
            string).

    Returns:
        A ``StringRule`` that passes for UUID strings of a supported version.

    Example:
        >>> rule = uuid()
        >>> rule.validate("123e4567-e89b-42d3-a456-426614174000")
        True
        >>> rule.validate("123e4567-e89b-12d3-a456-426614174000")
        False
        >>> uuid([1, 4]).validate("123e4567-e89b-12d3-a456-426614174000")
        True
    """
    safe_versions = sorted(
        {
            int(version)
            for version in ([4] if versions is None else versions)
            if not isinstance(version, bool) and version in _VALID_UUID_VERSIONS
        }
    )
    if not safe_versions:
        return StringRule(validate=lambda _value: False, message=message)

    version_class = "".join(str(version) for version in safe_versions)
    pattern = re.compile(
        rf"[0-9a-f]{{8}}-?[0-9a-f]{{4}}-?[{version_class}][0-9a-f]{{3}}"
        r"-?[89ab][0-9a-f]{3}-?[0-9a-f]{12}",
        re.IGNORECASE | re.ASCII,
    )
    return StringRule(
        validate=lambda value: pattern.fullmatch(value) is not None,
        message=message,
    )
