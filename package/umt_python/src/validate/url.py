from src.url.is_absolute_url import is_absolute_url

from .string import StringRule


def url(message: str = "") -> StringRule:
    """
    Creates a rule for checking if a string is an absolute URL.

    A string passes when it starts with an RFC 3986 scheme followed by a
    colon, as checked by ``is_absolute_url``.

    Args:
        message: Custom error message for validation failure (default: empty
            string).

    Returns:
        A ``StringRule`` that passes for absolute URLs.

    Example:
        >>> rule = url()
        >>> rule.validate("https://example.com")
        True
        >>> rule.validate("mailto:user@host")
        True
        >>> rule.validate("/path/to/page")
        False
    """
    return StringRule(validate=is_absolute_url, message=message)
