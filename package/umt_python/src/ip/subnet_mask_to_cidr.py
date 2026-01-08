import re


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

    binary_octets = []
    for octet in octets:
        try:
            num = int(octet)
        except ValueError:
            raise ValueError("Invalid subnet mask format")

        if num < 0 or num > 255:
            raise ValueError("Invalid subnet mask format")

        binary_octets.append(bin(num)[2:].zfill(8))

    binary_string = "".join(binary_octets)

    if not re.match(r"^1*0*$", binary_string):
        raise ValueError("Invalid subnet mask: must be consecutive 1s followed by 0s")

    return binary_string.count("1")
