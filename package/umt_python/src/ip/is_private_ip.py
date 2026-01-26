from .is_in_range import is_in_range


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

    private_ranges = [
        {"network": "10.0.0.0", "cidr": 8},
        {"network": "172.16.0.0", "cidr": 12},
        {"network": "192.168.0.0", "cidr": 16},
    ]

    try:
        return any(is_in_range(ip, r["network"], r["cidr"]) for r in private_ranges)
    except Exception as error:
        raise ValueError(f"Invalid IP address: {error}") from error
