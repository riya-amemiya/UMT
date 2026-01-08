from .decode_base58 import decode_base58


def decode_base58_to_string(input_data: str) -> str:
    """
    Decodes a Base58 string to a UTF-8 string.

    Args:
        input_data: Base58 encoded string

    Returns:
        Decoded string

    Example:
        >>> decode_base58_to_string("9Ajdvzr")
        'Hello'
    """
    return decode_base58(input_data).decode("utf-8")
