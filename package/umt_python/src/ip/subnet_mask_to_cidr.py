def subnet_mask_to_cidr(subnet_mask: str) -> int:
    """
    Converts a subnet mask to CIDR notation.

    Args:
        subnet_mask: IPv4 subnet mask (e.g., "255.255.255.0")

    Returns:
        CIDR notation (0-32)

    Raises:
        ValueError: If subnet mask is invalid

    Example:
        >>> subnet_mask_to_cidr("255.255.255.0")
        24
        >>> subnet_mask_to_cidr("255.255.0.0")
        16
    """
    if not subnet_mask:
        raise ValueError("Subnet mask is required")

    octets = subnet_mask.split(".")
    if len(octets) != 4:
        raise ValueError("Invalid subnet mask format")

    try:
        nums = [int(octet) for octet in octets]
    except ValueError:
        raise ValueError("Invalid subnet mask format") from None

    mask = 0
    for num in nums:
        if num < 0 or num > 255:
            raise ValueError("Invalid subnet mask format")
        mask = (mask << 8) | num

    # Host bits of a valid prefix mask are 0 or 2^n-1, so host & (host + 1)
    # is 0. 0xFFFFFFFF ^ mask is a 32-bit invert (Python ints are unbounded).
    host = 0xFFFFFFFF ^ mask
    if host & (host + 1) != 0:
        raise ValueError("Invalid subnet mask: must be consecutive 1s followed by 0s")

    return mask.bit_count()
