from .ip_to_long import ip_to_long


def is_in_range(remote_ip: str, network_ip: str, cidr: int) -> bool:
    """
    Checks if an IP address is within a specified network range.

    Args:
        remote_ip: IP address to check (e.g., "192.168.1.1")
        network_ip: Network IP address (e.g., "192.168.0.0")
        cidr: CIDR notation (0-32)

    Returns:
        True if the IP is in range, False otherwise

    Raises:
        ValueError: If any parameter is invalid

    Example:
        >>> is_in_range("192.168.1.100", "192.168.0.0", 16)
        True
        >>> is_in_range("10.0.0.1", "192.168.0.0", 16)
        False
    """
    if not remote_ip:
        raise ValueError("Remote IP address is required")
    if not network_ip:
        raise ValueError("Network IP address is required")

    if not isinstance(cidr, int) or cidr < 0 or cidr > 32:
        raise ValueError("CIDR must be an integer between 0 and 32")

    try:
        remote_long = ip_to_long(remote_ip)
        network_long = ip_to_long(network_ip)

        if cidr == 0:
            return True
        if cidr == 32:
            return remote_long == network_long

        shift = 32 - cidr
        mask = (0xFFFFFFFF >> shift) << shift
        return (remote_long & mask) == (network_long & mask)
    except Exception as error:
        raise ValueError(f"Invalid IP address format: {error}")
