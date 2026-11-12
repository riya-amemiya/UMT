_BASE58_INDEX = {
    char: index
    for index, char in enumerate(
        "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"
    )
}


def decode_base58(input_data: str) -> bytes:
    """
    Decodes a Base58 string to bytes.

    Args:
        input_data: Base58 encoded string

    Returns:
        Decoded bytes

    Raises:
        ValueError: If the input contains invalid Base58 characters

    Example:
        >>> decode_base58("9Ajdvzr")
        b'Hello'
    """
    big_number = 0

    for char in input_data:
        value = _BASE58_INDEX.get(char)
        if value is None:
            raise ValueError(f"Invalid base58 character: {char}")
        big_number = big_number * 58 + value

    leading_ones = len(input_data) - len(input_data.lstrip("1"))

    return bytes(leading_ones) + big_number.to_bytes(
        (big_number.bit_length() + 7) // 8, "big"
    )
