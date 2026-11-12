import base64


def encode_base64_url(char: str) -> str:
    """
    Convert string to URL-safe Base64 (RFC 4648 section 5) without padding.

    Args:
        char: String to convert to URL-safe Base64.

    Returns:
        URL-safe Base64 encoded string using "-" and "_" and no "=" padding.

    Example:
        >>> encode_base64_url("Hello World")
        'SGVsbG8gV29ybGQ'
        >>> encode_base64_url("\\u00ff\\u00fe")
        'w7_Dvg'
    """
    return base64.urlsafe_b64encode(char.encode("utf-8")).decode("ascii").rstrip("=")
