import re
from dataclasses import dataclass
from typing import Literal

from ..math.js_string_to_number import JS_WHITESPACE

ParseEmailLevel = Literal["basic", "rfc822", "rfc2822", "rfc5321", "rfc5322"]

_MAX_EMAIL_LENGTH = 320
_JS_WHITESPACE_CLASS = f"[{JS_WHITESPACE}]"
_ASTRAL_RE = re.compile("[\U00010000-\U0010ffff]")


def _compile_email_pattern(source: str) -> re.Pattern[str]:
    return re.compile(source.replace(r"(?:\s|", f"(?:{_JS_WHITESPACE_CLASS}|"))


def _split_surrogate_pair(match: re.Match[str]) -> str:
    offset = ord(match.group()) - 0x10000
    return chr(0xD800 + (offset >> 10)) + chr(0xDC00 + (offset & 0x3FF))


def _join_surrogate_pairs(value: str) -> str:
    return value.encode("utf-16-le", "surrogatepass").decode(
        "utf-16-le", "surrogatepass"
    )


EMAIL_PATTERNS = {
    "basic": _compile_email_pattern(
        r"^(?P<local>[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+)@"
        r"(?P<domain>[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?"
        r"(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*)$"
    ),
    "rfc822": _compile_email_pattern(
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
    "rfc2822": _compile_email_pattern(
        r"^(?=[^\n\r\u2028\u2029]{1,998}\Z)(?!.*\.\.)"
        r"(?P<local>[a-zA-Z0-9!#$%&'*/=?^_`{|}~-]"
        r"(?:[a-zA-Z0-9!#$%&'*/=?^_`{|}~.+-]{0,62}"
        r"[a-zA-Z0-9!#$%&'*/=?^_`{|}~-])?)@"
        r"(?P<domain>[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?"
        r"(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*"
        r"\.[a-zA-Z]{2,})$"
    ),
    "rfc5321": _compile_email_pattern(
        r"^(?=[^\n\r\u2028\u2029]{1,256}\Z)(?=(?:[^@]{1,64})@)(?!.*\.\.)"
        r"(?P<local>(?:[a-zA-Z0-9!#$%&'*+/=?^_`{|}~-]+"
        r"(?:\.[a-zA-Z0-9!#$%&'*+/=?^_`{|}~-]+)*|"
        r'"(?:[^"\\]|\\[\s\S]){0,62}"))@'
        r"(?P<domain>[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?"
        r"(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)+|"
        r"\[(?:(?:[0-9]{1,3}\.){3}[0-9]{1,3}|IPv6:[0-9a-fA-F:]+)\])$"
    ),
    "rfc5322": _compile_email_pattern(
        r"^(?=[^\n\r\u2028\u2029]{1,998}\Z)"
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
    parts: EmailParts | None = None


def parse_email(email: str, options: ParseEmailOptions) -> ParseEmailResult:
    """
    Parses and validates an email address according to the specified RFC level.

    Every length limit counts UTF-16 code units like the TypeScript
    implementation, so a character outside the Basic Multilingual Plane counts
    as two.

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
    is_ascii = email.isascii()
    units = email if is_ascii else _ASTRAL_RE.sub(_split_surrogate_pair, email)
    if len(units) > _MAX_EMAIL_LENGTH:
        return ParseEmailResult(valid=False)

    level = options.level
    pattern = EMAIL_PATTERNS[level]
    match = pattern.fullmatch(units)

    if match is None:
        return ParseEmailResult(valid=False)

    groups = match.groupdict()
    if is_ascii:
        return ParseEmailResult(
            valid=True,
            parts=EmailParts(local=groups["local"], domain=groups["domain"]),
        )
    return ParseEmailResult(
        valid=True,
        parts=EmailParts(
            local=_join_surrogate_pairs(groups["local"]),
            domain=_join_surrogate_pairs(groups["domain"]),
        ),
    )
