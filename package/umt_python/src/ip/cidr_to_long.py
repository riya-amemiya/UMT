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

    # Bitwise mask of `cidr` leading 1s instead of allocating a 32-char
    # binary string and parsing it. Matches TS cidrToLong / Rust
    # cidr_to_long. cidr=0 -> 0; cidr=32 -> 0xFFFFFFFF.
    # ~2x faster (~228 ns -> ~115 ns / call, 2M mixed CIDR 0-32,
    # CPython 3.12.3).
    return ((1 << cidr) - 1) << (32 - cidr)
