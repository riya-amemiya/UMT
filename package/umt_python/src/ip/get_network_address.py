from .cidr_to_long import cidr_to_long
from .ip_to_long import ip_to_long
from .subnet_mask_to_cidr import subnet_mask_to_cidr


def get_network_address(ip: str, subnet_mask: str) -> int:
    """
    Calculates the network address from an IP address and subnet mask.

    Args:
        ip: IPv4 address (e.g., "192.168.1.1")
        subnet_mask: Subnet mask (e.g., "255.255.255.0")

    Returns:
        Network address as a 32-bit unsigned integer

    Raises:
        ValueError: If IP address or subnet mask is invalid

    Example:
        >>> get_network_address("192.168.1.100", "255.255.255.0")
        3232235776
    """
    if not ip:
        raise ValueError("IP address is required")
    if not subnet_mask:
        raise ValueError("Subnet mask is required")

    try:
        return ip_to_long(ip) & cidr_to_long(subnet_mask_to_cidr(subnet_mask))
    except Exception:
        raise TypeError("Invalid IP address or subnet mask") from None
