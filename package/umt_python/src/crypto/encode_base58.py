def encode_base58(input_data: str | bytes) -> str:
    """
    Encodes a string or bytes to Base58 format.

    Args:
        input_data: The input to encode (string or bytes)

    Returns:
        Base58 encoded string

    Example:
        >>> encode_base58("Hello")
        '9Ajdvzr'
    """
    alphabet = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"

    data = input_data.encode("utf-8") if isinstance(input_data, str) else input_data

    encoded = ""
    big_number = 0

    for byte in data:
        big_number = big_number * 256 + byte

    while big_number > 0:
        remainder = big_number % 58
        encoded = alphabet[remainder] + encoded
        big_number //= 58

    leading_zeros = 0
    for byte in data:
        if byte != 0:
            break
        leading_zeros += 1

    return "1" * leading_zeros + encoded
