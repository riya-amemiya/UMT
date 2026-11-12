import base64
import re

_BASE64_URL_RE = re.compile(r"[A-Za-z0-9_-]*")


def decode_base64_url(base64_url_string: str) -> str:
    """
    Converts URL-safe Base64 (RFC 4648 section 5) to string.

    Padding is optional. Missing "=" characters are added up to a multiple of
    four first, so partial padding such as "Zg=" is accepted.

    Args:
        base64_url_string: URL-safe Base64 encoded string, with or without "=" padding.

    Returns:
        Decoded string from URL-safe Base64.

    Raises:
        ValueError: When input is not a valid URL-safe Base64 string or the
            decoded bytes are not valid UTF-8.

    Example:
        >>> decode_base64_url("SGVsbG8gV29ybGQ")
        'Hello World'
        >>> decode_base64_url("SGVsbG8gV29ybGQ=")
        'Hello World'
    """
    padded = base64_url_string + "=" * (-len(base64_url_string) % 4)
    data = (
        padded.removesuffix("==") if padded.endswith("==") else padded.removesuffix("=")
    )
    if len(data) % 4 == 1 or _BASE64_URL_RE.fullmatch(data) is None:
        raise ValueError("Invalid Base64 string")
    try:
        return base64.urlsafe_b64decode(data + "=" * (-len(data) % 4)).decode("utf-8")
    except UnicodeDecodeError as e:
        raise ValueError("Invalid Base64 string") from e
