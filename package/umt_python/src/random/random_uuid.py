import secrets


def _bytes_to_uuid(byte_values: bytes) -> str:
    digits = byte_values[:16].hex()
    return f"{digits[:8]}-{digits[8:12]}-{digits[12:16]}-{digits[16:20]}-{digits[20:]}"


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
