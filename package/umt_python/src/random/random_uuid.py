import secrets

_DIGITS = "0123456789abcdef"


def _bytes_to_uuid(byte_values: bytes) -> str:
    result = ""
    for index in range(16):
        if index in (4, 6, 8, 10):
            result += "-"
        byte = byte_values[index]
        result += _DIGITS[byte >> 4] + _DIGITS[byte & 0xF]
    return result


def random_uuid() -> str:
    """
    Generates a UUID v4 string in canonical 8-4-4-4-12 hex form.

    Random bytes are sourced from a cryptographically secure generator,
    with the version (4) and variant bits set to match the UUID v4 layout.

    Returns:
        A UUID v4 string.

    Example:
        >>> import re
        >>> pattern = re.compile(
        ...     r"^[\\da-f]{8}-[\\da-f]{4}-4[\\da-f]{3}-[89ab][\\da-f]{3}-[\\da-f]{12}$",
        ...     re.IGNORECASE,
        ... )
        >>> bool(pattern.match(random_uuid()))
        True
        >>> random_uuid() != random_uuid()
        True
    """
    byte_values = bytearray(secrets.token_bytes(16))
    byte_values[6] = (byte_values[6] & 0x0F) | 0x40
    byte_values[8] = (byte_values[8] & 0x3F) | 0x80
    return _bytes_to_uuid(bytes(byte_values))
