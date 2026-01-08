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
    alphabet = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"
    big_number = 0

    for char in input_data:
        value = alphabet.find(char)
        if value == -1:
            raise ValueError(f"Invalid base58 character: {char}")
        big_number = big_number * 58 + value

    result = []
    while big_number > 0:
        result.insert(0, big_number % 256)
        big_number //= 256

    leading_ones = 0
    for char in input_data:
        if char != "1":
            break
        leading_ones += 1

    return bytes([0] * leading_ones + result)
