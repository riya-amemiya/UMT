from .cidr_to_long import cidr_to_long
from .long_to_ip import long_to_ip


def cidr_to_subnet_mask(cidr: int) -> str:
    """
    Converts CIDR notation to a subnet mask.

    Args:
        cidr: CIDR notation (0-32)

    Returns:
        Subnet mask in IPv4 format (e.g., "255.255.255.0")

    Raises:
        ValueError: If CIDR is not between 0 and 32

    Example:
        >>> cidr_to_subnet_mask(24)
        '255.255.255.0'
        >>> cidr_to_subnet_mask(16)
        '255.255.0.0'
    """
    return long_to_ip(cidr_to_long(cidr))
