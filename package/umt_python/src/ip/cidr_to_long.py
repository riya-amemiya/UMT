def cidr_to_long(cidr: int) -> int:
    """
    Converts CIDR notation to a subnet mask number.

    Args:
        cidr: CIDR notation (0-32)

    Returns:
        Subnet mask as a 32-bit number

    Raises:
        ValueError: If CIDR is not between 0 and 32

    Example:
        >>> cidr_to_long(24)
        4294967040
        >>> cidr_to_long(32)
        4294967295
        >>> cidr_to_long(0)
        0
    """
    if not isinstance(cidr, int) or cidr < 0 or cidr > 32:
        raise ValueError("CIDR must be an integer between 0 and 32")

    binary_str = "1" * cidr + "0" * (32 - cidr)
    return int(binary_str, 2)
