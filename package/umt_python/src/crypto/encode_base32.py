from typing import Union


def encode_base32(input_data: Union[str, bytes]) -> str:
    """
    Encodes a string or bytes to Base32 format.

    Args:
        input_data: The input to encode (string or bytes)

    Returns:
        Base32 encoded string

    Example:
        >>> encode_base32("Hello")
        'JBSWY3DP'
    """
    alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567"

    if isinstance(input_data, str):
        data = input_data.encode("utf-8")
    else:
        data = input_data

    result = ""
    buffer = 0
    buffer_length = 0

    for byte in data:
        buffer = (buffer << 8) | byte
        buffer_length += 8

        while buffer_length >= 5:
            buffer_length -= 5
            result += alphabet[(buffer >> buffer_length) & 0x1F]

    if buffer_length > 0:
        result += alphabet[(buffer << (5 - buffer_length)) & 0x1F]

    padding_length = (8 - (len(result) % 8)) % 8
    result += "=" * padding_length

    return result
