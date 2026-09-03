from .ip_to_long import ip_to_long


def ip_to_binary_string(ip: str) -> str:
    """
    Converts an IPv4 address to its binary string representation.

    Args:
        ip: IPv4 address (e.g., "192.168.1.1")

    Returns:
        Binary string representation (32 bits)

    Raises:
        ValueError: If IP address is invalid

    Example:
        >>> ip_to_binary_string("192.168.1.1")
        '11000000101010000000000100000001'
        >>> ip_to_binary_string("0.0.0.0")
        '00000000000000000000000000000000'
    """
    return f"{ip_to_long(ip):032b}"
