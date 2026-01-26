from .decode_base32 import decode_base32


def decode_base32_to_string(input_data: str) -> str:
    """
    Decodes a Base32 string to a UTF-8 string.

    Args:
        input_data: Base32 encoded string

    Returns:
        Decoded string

    Example:
        >>> decode_base32_to_string("JBSWY3DP")
        'Hello'
    """
    return decode_base32(input_data).decode("utf-8")
