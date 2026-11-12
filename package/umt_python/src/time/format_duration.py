import math

_DURATION_UNITS: tuple[tuple[str, int, int | None], ...] = (
    ("d", 86400000, None),
    ("h", 3600000, 24),
    ("m", 60000, 60),
    ("s", 1000, 60),
    ("ms", 1, 1000),
)


def format_duration(milliseconds: int | float) -> str:
    """
    Format a millisecond duration as a compact human-readable string.

    Units are days (d), hours (h), minutes (m), seconds (s) and
    milliseconds (ms), separated by a space. Zero units are omitted,
    fractional milliseconds are truncated, and a zero duration returns "0s".
    Negative durations keep a leading "-".

    Args:
        milliseconds: Duration in milliseconds

    Returns:
        The formatted duration

    Example:
        >>> format_duration(5400000)
        '1h 30m'
        >>> format_duration(45000)
        '45s'
        >>> format_duration(183600000)
        '2d 3h'
        >>> format_duration(1500)
        '1s 500ms'
        >>> format_duration(-90000)
        '-1m 30s'
        >>> format_duration(0)
        '0s'
    """
    total = math.trunc(abs(milliseconds))
    values = [
        (label, total // size if modulus is None else total // size % modulus)
        for label, size, modulus in _DURATION_UNITS
    ]
    parts = [f"{value}{label}" for label, value in values if value > 0]
    if not parts:
        return "0s"
    sign = "-" if milliseconds < 0 else ""
    return sign + " ".join(parts)
