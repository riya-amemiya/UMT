import re
from typing import TypedDict

from .cidr_to_long import cidr_to_long
from .ip_to_long import ip_to_long
from .long_to_ip import long_to_ip

_PREFIX_LENGTH_RE = re.compile(r"3[0-2]|[12]?[0-9]")


class ParsedCidr(TypedDict):
    """Network address and prefix length parsed from CIDR notation."""

    network_address: str
    prefix_length: int


def parse_cidr(cidr: str) -> ParsedCidr:
    """
    Parses CIDR notation into its network address and prefix length.

    The host bits of the address are cleared, so "192.168.1.130/25" gives
    the network address "192.168.1.128".

    Args:
        cidr: CIDR notation (e.g., "192.168.0.0/24")

    Returns:
        Dictionary with the network address and the prefix length (0-32)

    Raises:
        ValueError: If the address or the prefix length is invalid

    Example:
        >>> parse_cidr("192.168.0.0/24")
        {'network_address': '192.168.0.0', 'prefix_length': 24}
        >>> parse_cidr("192.168.1.130/25")
        {'network_address': '192.168.1.128', 'prefix_length': 25}
    """
    parts = cidr.split("/")
    if len(parts) != 2 or _PREFIX_LENGTH_RE.fullmatch(parts[1]) is None:
        raise ValueError("Invalid CIDR notation")

    prefix_length = int(parts[1])
    return {
        "network_address": long_to_ip(
            ip_to_long(parts[0]) & cidr_to_long(prefix_length)
        ),
        "prefix_length": prefix_length,
    }
