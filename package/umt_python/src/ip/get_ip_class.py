import re

_LEADING_INTEGER_RE = re.compile(
    r"[\t\n\v\f\r \N{NO-BREAK SPACE}\N{OGHAM SPACE MARK}\N{EN QUAD}-\N{HAIR SPACE}"
    r"\N{LINE SEPARATOR}\N{PARAGRAPH SEPARATOR}\N{NARROW NO-BREAK SPACE}"
    r"\N{MEDIUM MATHEMATICAL SPACE}\N{IDEOGRAPHIC SPACE}\N{ZERO WIDTH NO-BREAK SPACE}]*"
    r"([+-]?[0-9]+)"
)


def get_ip_class(ip: str) -> str:
    """
    Gets the IP address class (A, B, C, D, or E).

    The first octet is read like JavaScript ``parseInt(octet, 10)``: leading
    whitespace and a sign are accepted, and parsing stops at the first
    character that is not an ASCII digit.

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

    leading_integer = _LEADING_INTEGER_RE.match(parts[0])
    if leading_integer is None:
        return ""

    first_octet = int(leading_integer.group(1))
    if first_octet <= 0 or first_octet > 255:
        return ""

    if first_octet < 128:
        return "A"
    if first_octet < 192:
        return "B"
    if first_octet < 224:
        return "C"
    if first_octet < 240:
        return "D"
    return "E"
