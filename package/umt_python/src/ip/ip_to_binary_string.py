import re


def ip_to_binary_string(ip: str) -> str:
    """
    Converts an IPv4 address to its binary string representation.

    Args:
        ip: IPv4 address (e.g., "192.168.1.1")

    Returns:
        Binary string representation (32 bits)

    Raises:
        ValueError: If IP address is invalid

    Example:
        >>> ip_to_binary_string("192.168.1.1")
        '11000000101010000000000100000001'
        >>> ip_to_binary_string("0.0.0.0")
        '00000000000000000000000000000000'
    """
    if not ip:
        raise ValueError("IP address is required")

    if re.search(r"[^0-9.]", ip):
        raise ValueError("Invalid IP address format")

    parts = ip.split(".")
    if len(parts) != 4:
        raise ValueError("Invalid IP address format")

    for octet in parts:
        if not octet or (len(octet) > 1 and octet.startswith("0")):
            raise ValueError("Invalid IP address format")

        try:
            num = int(octet)
        except ValueError:
            raise ValueError("Invalid IP address format")

        if num < 0 or num > 255:
            raise ValueError("Invalid IP address format")

    return "".join(bin(int(octet))[2:].zfill(8) for octet in parts)
