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

    ip_parts = ip.split(".")
    if len(ip_parts) != 4:
        raise TypeError("Invalid IP address or subnet mask")

    for part in ip_parts:
        try:
            num = int(part)
            if num < 0 or num > 255:
                raise TypeError("Invalid IP address or subnet mask")
        except ValueError:
            raise TypeError("Invalid IP address or subnet mask")

    mask_parts = subnet_mask.split(".")
    if len(mask_parts) != 4:
        raise TypeError("Invalid IP address or subnet mask")

    for part in mask_parts:
        try:
            num = int(part)
            if num < 0 or num > 255:
                raise TypeError("Invalid IP address or subnet mask")
        except ValueError:
            raise TypeError("Invalid IP address or subnet mask")

    try:
        network_address = ip_to_long(ip) & cidr_to_long(
            subnet_mask_to_cidr(subnet_mask)
        )
        return network_address
    except Exception:
        raise TypeError("Invalid IP address or subnet mask")
