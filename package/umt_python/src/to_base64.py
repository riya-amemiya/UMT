import base64

def to_base64(char: str) -> str:
    """
    Convert string to Base64.

    Args:
        char: String to convert to Base64.

    Returns:
        Base64 encoded string.

    Example:
        >>> to_base64("Hello World")
        'SGVsbG8gV29ybGQ='
    """
    return base64.b64encode(char.encode('utf-8')).decode('utf-8')
