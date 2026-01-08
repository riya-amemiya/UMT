import re
from typing import Optional, Literal
from dataclasses import dataclass


ParseEmailLevel = Literal["basic", "rfc822", "rfc2822", "rfc5321", "rfc5322"]


EMAIL_PATTERNS = {
    "basic": re.compile(
        r"^(?P<local>[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+)@"
        r"(?P<domain>[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?"
        r"(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*)$"
    ),
    "rfc822": re.compile(
        r"^(?:\s|\((?:[^()\\]|\\[\s\S])*(?:\((?:[^()\\]|\\[\s\S])*\)"
        r"(?:[^()\\]|\\[\s\S])*)*\))*"
        r'(?P<local>"(?:[^"\\]|\\[\s\S]){0,62}"|'
        r"[a-zA-Z0-9!#$%&'*/=?^_`{|}~-]{1,64}"
        r"(?:\.[a-zA-Z0-9!#$%&'*/=?^_`{|}~-]+)*)"
        r"(?:\s|\((?:[^()\\]|\\[\s\S])*(?:\((?:[^()\\]|\\[\s\S])*\)"
        r"(?:[^()\\]|\\[\s\S])*)*\))*@"
        r"(?:\s|\((?:[^()\\]|\\[\s\S])*(?:\((?:[^()\\]|\\[\s\S])*\)"
        r"(?:[^()\\]|\\[\s\S])*)*\))*"
        r"(?P<domain>[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?"
        r"(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*)"
        r"(?:\s|\((?:[^()\\]|\\[\s\S])*(?:\((?:[^()\\]|\\[\s\S])*\)"
        r"(?:[^()\\]|\\[\s\S])*)*\))*$"
    ),
    "rfc2822": re.compile(
        r"^(?=.{1,998}$)(?!.*\.\.)"
        r"(?P<local>[a-zA-Z0-9!#$%&'*/=?^_`{|}~-]"
        r"(?:[a-zA-Z0-9!#$%&'*/=?^_`{|}~.+-]{0,62}"
        r"[a-zA-Z0-9!#$%&'*/=?^_`{|}~-])?)@"
        r"(?P<domain>[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?"
        r"(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*"
        r"\.[a-zA-Z]{2,})$"
    ),
    "rfc5321": re.compile(
        r"^(?=.{1,256}$)(?=(?:[^@]{1,64})@)(?!.*\.\.)"
        r"(?P<local>(?:[a-zA-Z0-9!#$%&'*+/=?^_`{|}~-]+"
        r"(?:\.[a-zA-Z0-9!#$%&'*+/=?^_`{|}~-]+)*|"
        r'"(?:[^"\\]|\\[\s\S]){0,62}"))@'
        r"(?P<domain>[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?"
        r"(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)+|"
        r"\[(?:(?:[0-9]{1,3}\.){3}[0-9]{1,3}|IPv6:[0-9a-fA-F:]+)\])$"
    ),
    "rfc5322": re.compile(
        r"^(?=.{1,998}$)"
        r"(?:\s|\((?:[^()\\]|\\[\s\S])*(?:\((?:[^()\\]|\\[\s\S])*\)"
        r"(?:[^()\\]|\\[\s\S])*)*\))*"
        r'(?P<local>"(?:[^"\\]|\\[\s\S]){0,62}"'
        r'(?:\."(?:[^"\\]|\\[\s\S]){0,62}")*|'
        r'"(?:[^"\\]|\\[\s\S]){0,62}"'
        r"(?:\.[a-zA-Z0-9!#$%&'*/=?^_`{|}~+-]{1,64}"
        r"(?:\.[a-zA-Z0-9!#$%&'*/=?^_`{|}~+-]{1,64})*)+|"
        r"[a-zA-Z0-9!#$%&'*/=?^_`{|}~+-]{1,64}"
        r"(?:\.[a-zA-Z0-9!#$%&'*/=?^_`{|}~+-]{1,64})*"
        r'(?:\."(?:[^"\\]|\\[\s\S]){0,62}")+|'
        r"[a-zA-Z0-9!#$%&'*/=?^_`{|}~+-]{1,64}"
        r"(?:(?:\s|\((?:[^()\\]|\\[\s\S])*(?:\((?:[^()\\]|\\[\s\S])*\)"
        r"(?:[^()\\]|\\[\s\S])*)*\))*\."
        r"(?:\s|\((?:[^()\\]|\\[\s\S])*(?:\((?:[^()\\]|\\[\s\S])*\)"
        r"(?:[^()\\]|\\[\s\S])*)*\))*"
        r"[a-zA-Z0-9!#$%&'*/=?^_`{|}~+-]{1,64})*)"
        r"(?:\s|\((?:[^()\\]|\\[\s\S])*(?:\((?:[^()\\]|\\[\s\S])*\)"
        r"(?:[^()\\]|\\[\s\S])*)*\))*@"
        r"(?:\s|\((?:[^()\\]|\\[\s\S])*(?:\((?:[^()\\]|\\[\s\S])*\)"
        r"(?:[^()\\]|\\[\s\S])*)*\))*"
        r"(?P<domain>[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?"
        r"(?:(?:\s|\((?:[^()\\]|\\[\s\S])*(?:\((?:[^()\\]|\\[\s\S])*\)"
        r"(?:[^()\\]|\\[\s\S])*)*\))*\."
        r"(?:\s|\((?:[^()\\]|\\[\s\S])*(?:\((?:[^()\\]|\\[\s\S])*\)"
        r"(?:[^()\\]|\\[\s\S])*)*\))*"
        r"[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)+|"
        r"\[(?:(?:[0-9]{1,3}\.){3}[0-9]{1,3}|IPv6:[0-9a-fA-F:]+)\])"
        r"(?:\s|\((?:[^()\\]|\\[\s\S])*(?:\((?:[^()\\]|\\[\s\S])*\)"
        r"(?:[^()\\]|\\[\s\S])*)*\))*$"
    ),
}


@dataclass
class ParseEmailOptions:
    """
    Options for parse_email function.

    Attributes:
        level: The validation level to use.
    """

    level: ParseEmailLevel


@dataclass
class EmailParts:
    """
    Parsed email parts.

    Attributes:
        local: The local part of the email address.
        domain: The domain part of the email address.
    """

    local: str
    domain: str


@dataclass
class ParseEmailResult:
    """
    Result of parse_email function.

    Attributes:
        valid: Whether the email is valid.
        parts: The parsed email parts, if valid.
    """

    valid: bool
    parts: Optional[EmailParts] = None


def parse_email(email: str, options: ParseEmailOptions) -> ParseEmailResult:
    """
    Parses and validates an email address according to the specified RFC level.

    Args:
        email: The email address to parse.
        options: Parse options containing the validation level.

    Returns:
        A ParseEmailResult containing validity and parsed parts.

    Example:
        >>> result = parse_email("test@example.com", ParseEmailOptions(level="basic"))
        >>> result.valid
        True
        >>> result.parts.local
        'test'
        >>> result.parts.domain
        'example.com'
    """
    level = options.level
    pattern = EMAIL_PATTERNS[level]
    match = pattern.match(email)

    if match is None:
        return ParseEmailResult(valid=False)

    groups = match.groupdict()
    return ParseEmailResult(
        valid=True,
        parts=EmailParts(local=groups["local"], domain=groups["domain"]),
    )
