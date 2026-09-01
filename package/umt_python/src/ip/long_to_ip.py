import math


def long_to_ip(long: int | float) -> str:
    """
    Converts a 32-bit number to an IPv4 address.

    Args:
        long: 32-bit unsigned integer to convert

    Returns:
        IPv4 address (e.g., "192.168.1.1")

    Raises:
        ValueError: If input is not a valid 32-bit unsigned integer

    Example:
        >>> long_to_ip(3232235777)
        '192.168.1.1'
        >>> long_to_ip(0)
        '0.0.0.0'
    """
    if not math.isfinite(long) or long < 0 or long > 0xFFFFFFFF or long != int(long):
        raise ValueError("Input must be a valid 32-bit unsigned integer")

    value = int(long)
    return f"{(value >> 24) & 0xFF}.{(value >> 16) & 0xFF}.{(value >> 8) & 0xFF}.{value & 0xFF}"
