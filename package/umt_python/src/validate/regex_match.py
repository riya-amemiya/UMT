import re

from .string import StringRule


def regex_match(pattern: re.Pattern[str], message: str = "") -> StringRule:
    """
    Creates a rule for checking if a string matches a regular expression
    pattern.

    The pattern is searched anywhere in the string, matching JavaScript's
    ``RegExp.prototype.test``.

    Args:
        pattern: Compiled regular expression to match against.
        message: Custom error message for validation failure (default: empty
            string).

    Returns:
        A ``StringRule`` that passes for strings matching the pattern.

    Example:
        >>> import re
        >>> rule = regex_match(re.compile(r"^[0-9]+$"))
        >>> rule.validate("123")
        True
        >>> rule.validate("12a")
        False
    """
    return StringRule(
        validate=lambda value: pattern.search(value) is not None, message=message
    )
