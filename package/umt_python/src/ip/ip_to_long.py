from .ip_to_binary_string import ip_to_binary_string


def ip_to_long(ip: str) -> int:
    """
    Converts an IPv4 address to a 32-bit number.

    Args:
        ip: IPv4 address to convert (e.g., "192.168.1.1")

    Returns:
        32-bit unsigned integer

    Raises:
        ValueError: If IP address is invalid

    Example:
        >>> ip_to_long("192.168.1.1")
        3232235777
        >>> ip_to_long("0.0.0.0")
        0
    """
    return int(ip_to_binary_string(ip), 2)
