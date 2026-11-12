def utf16_length(value: str) -> int:
    """
    Returns the length of a string in UTF-16 code units.

    This matches the JavaScript ``String.prototype.length`` property, where
    characters outside the Basic Multilingual Plane count as two units.

    Args:
        value: String to measure.

    Returns:
        The number of UTF-16 code units in the string.

    Example:
        >>> utf16_length("abc")
        3
        >>> utf16_length("\U0001f600")
        2
    """
    if value.isascii():
        return len(value)
    return len(value.encode("utf-16-le", "surrogatepass")) // 2
