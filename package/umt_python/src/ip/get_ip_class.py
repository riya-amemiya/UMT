def get_ip_class(ip: str) -> str:
    """
    Gets the IP address class (A, B, C, D, or E).

    Args:
        ip: IPv4 address

    Returns:
        IP class ('A', 'B', 'C', 'D', 'E', or empty string for invalid IP)

    Example:
        >>> get_ip_class("10.0.0.1")
        'A'
        >>> get_ip_class("172.16.0.1")
        'B'
        >>> get_ip_class("192.168.1.1")
        'C'
    """
    if not ip:
        return ""

    parts = ip.split(".")
    if len(parts) != 4:
        return ""

    try:
        first_octet = int(parts[0])
    except ValueError:
        return ""

    if first_octet < 0 or first_octet > 255:
        return ""

    if first_octet == 0:
        return ""
    elif first_octet < 128:
        return "A"
    elif first_octet < 192:
        return "B"
    elif first_octet < 224:
        return "C"
    elif first_octet < 240:
        return "D"
    else:
        return "E"
