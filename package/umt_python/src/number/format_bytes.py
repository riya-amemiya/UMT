import math

_BYTE_UNITS = ("B", "KB", "MB", "GB", "TB", "PB")


def format_bytes(bytes_: int | float, decimals: int = 2) -> str:
    """
    Formats a byte count as a human-readable string using base 1024.

    The unit is the largest of B, KB, MB, GB, TB and PB that does not exceed
    the absolute value (values of a PB or more stay in PB). The value is
    rounded to `decimals` places with JavaScript `Math.round` semantics and
    printed without trailing zeros.

    Args:
        bytes_: The number of bytes (may be negative or fractional).
        decimals: The number of decimal places (default 2).

    Returns:
        The formatted size, such as "1.5 KB".

    Example:
        >>> format_bytes(0)
        '0 B'
        >>> format_bytes(1024)
        '1 KB'
        >>> format_bytes(1536)
        '1.5 KB'
        >>> format_bytes(1234567)
        '1.18 MB'
        >>> format_bytes(1234567, 0)
        '1 MB'
        >>> format_bytes(-2048)
        '-2 KB'
    """
    absolute = abs(bytes_)
    index = max(
        (
            unit_index
            for unit_index in range(len(_BYTE_UNITS))
            if absolute >= 1024**unit_index
        ),
        default=0,
    )
    factor = 10**decimals
    value = math.floor(bytes_ / 1024**index * factor + 0.5) / factor
    text = str(int(value)) if value.is_integer() else repr(value)
    return f"{text} {_BYTE_UNITS[index]}"
