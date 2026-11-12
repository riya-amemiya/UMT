import re

_HEX_RE = re.compile(r"(?:[0-9A-Fa-f]{2})*")


def decode_hex(input_data: str) -> bytes:
    """
    Decodes a hexadecimal string to bytes.

    Args:
        input_data: Hexadecimal string (lowercase or uppercase, two characters per byte)

    Returns:
        Decoded bytes

    Raises:
        ValueError: If the input has an odd length or contains a non-hex character

    Example:
        >>> decode_hex("48656c6c6f")
        b'Hello'
        >>> decode_hex("00FF")
        b'\\x00\\xff'
    """
    if _HEX_RE.fullmatch(input_data) is None:
        raise ValueError("Invalid hex string")
    return bytes.fromhex(input_data)
