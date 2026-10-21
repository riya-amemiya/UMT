from .cidr_to_long import cidr_to_long
from .ip_to_long import ip_to_long

_PRIVATE_RANGES = (
    (ip_to_long("10.0.0.0"), cidr_to_long(8)),
    (ip_to_long("172.16.0.0"), cidr_to_long(12)),
    (ip_to_long("192.168.0.0"), cidr_to_long(16)),
)


def is_private_ip(ip: str) -> bool:
    """
    Checks if an IP address is within private IP ranges.

    Args:
        ip: IP address to check (e.g., "192.168.1.1")

    Returns:
        True if the IP is private, False otherwise

    Raises:
        ValueError: If IP address is invalid

    Example:
        >>> is_private_ip("192.168.1.1")
        True
        >>> is_private_ip("8.8.8.8")
        False
        >>> is_private_ip("10.0.0.1")
        True
    """
    if not ip:
        raise ValueError("IP address is required")

    try:
        remote_long = ip_to_long(ip)
    except Exception as error:
        raise ValueError(f"Invalid IP address: {error}") from error

    return any((remote_long & mask) == network for network, mask in _PRIVATE_RANGES)
