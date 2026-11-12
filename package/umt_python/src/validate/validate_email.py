from .parse_email import ParseEmailOptions, parse_email
from .string import StringRule


def validate_email(
    message: str = "",
    options: ParseEmailOptions | None = None,
) -> StringRule:
    """
    Creates a rule for checking if a string is a valid email address.

    Args:
        message: Custom error message for validation failure (default: empty
            string).
        options: Parse options containing the validation level (default: the
            ``"basic"`` level).

    Returns:
        A ``StringRule`` that passes for valid email addresses.

    Example:
        >>> rule = validate_email()
        >>> rule.validate("user@example.com")
        True
        >>> rule.validate("invalid-email")
        False
    """
    resolved = options if options is not None else ParseEmailOptions(level="basic")
    return StringRule(
        validate=lambda value: parse_email(value, resolved).valid,
        message=message,
    )
