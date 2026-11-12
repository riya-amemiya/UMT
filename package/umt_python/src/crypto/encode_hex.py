def encode_hex(input_data: str | bytes) -> str:
    """
    Encodes a string or bytes to lowercase hexadecimal.

    Args:
        input_data: The input to encode (strings are UTF-8 encoded first)

    Returns:
        Lowercase hexadecimal string with two characters per byte

    Example:
        >>> encode_hex("Hello")
        '48656c6c6f'
        >>> encode_hex(b"\\x00\\xff")
        '00ff'
    """
    data = input_data.encode("utf-8") if isinstance(input_data, str) else input_data
    return data.hex()
