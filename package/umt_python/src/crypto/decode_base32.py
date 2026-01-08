def decode_base32(input_data: str) -> bytes:
    """
    Decodes a Base32 string to bytes.

    Args:
        input_data: Base32 encoded string

    Returns:
        Decoded bytes

    Raises:
        ValueError: If the input contains invalid Base32 characters

    Example:
        >>> decode_base32("JBSWY3DP")
        b'Hello'
    """
    alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567"
    cleaned_input = input_data.replace("=", "")
    result = []
    buffer = 0
    buffer_length = 0

    for char in cleaned_input:
        value = alphabet.find(char)
        if value == -1:
            raise ValueError(f"Invalid base32 character: {char}")

        buffer = (buffer << 5) | value
        buffer_length += 5

        if buffer_length >= 8:
            buffer_length -= 8
            result.append((buffer >> buffer_length) & 0xFF)

    return bytes(result)
