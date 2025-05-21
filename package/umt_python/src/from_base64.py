import base64

def from_base64(base64_string: str) -> str:
    """
    Converts Base64 to string.

    Args:
        base64_string: Base64 encoded string.

    Returns:
        Decoded string from Base64.

    Raises:
        ValueError: When input is not a valid Base64 string.

    Example:
        >>> from_base64("SGVsbG8gV29ybGQ=")
        'Hello World'
    """
    if not base64_string:
        return ""
    try:
        return base64.b64decode(base64_string.encode('utf-8')).decode('utf-8')
    except Exception as e:
        raise ValueError("Invalid Base64 string") from e
