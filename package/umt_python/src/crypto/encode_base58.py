_BASE58_ALPHABET = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"


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
    data = input_data.encode("utf-8") if isinstance(input_data, str) else input_data

    big_number = int.from_bytes(data, "big")
    chars: list[str] = []
    while big_number > 0:
        big_number, remainder = divmod(big_number, 58)
        chars.append(_BASE58_ALPHABET[remainder])
    chars.reverse()

    leading_zeros = len(data) - len(data.lstrip(b"\x00"))

    return "1" * leading_zeros + "".join(chars)
