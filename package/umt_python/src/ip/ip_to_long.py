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
    if not ip:
        raise ValueError("IP address is required")

    parts = ip.split(".")
    if len(parts) != 4:
        raise ValueError("Invalid IP address format")

    result = 0
    for octet in parts:
        if (
            not octet.isascii()
            or not octet.isdigit()
            or (len(octet) > 1 and octet[0] == "0")
        ):
            raise ValueError("Invalid IP address format")

        num = int(octet)
        if num > 255:
            raise ValueError("Invalid IP address format")

        result = (result << 8) | num

    return result
