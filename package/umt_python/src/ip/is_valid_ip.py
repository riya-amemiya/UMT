from .ip_to_long import ip_to_long


def is_valid_ip(ip: str) -> bool:
    """
    Checks whether a string is a valid IPv4 address in dotted-quad notation.

    Accepts exactly four ASCII decimal octets in the range 0-255 without
    leading zeros, the same input that ``ip_to_long`` accepts.

    Args:
        ip: String to check (e.g., "192.168.1.1")

    Returns:
        True if the string is a valid IPv4 address, False otherwise

    Example:
        >>> is_valid_ip("192.168.1.1")
        True
        >>> is_valid_ip("192.168.01.1")
        False
        >>> is_valid_ip("256.0.0.1")
        False
    """
    try:
        ip_to_long(ip)
    except ValueError:
        return False
    return True
