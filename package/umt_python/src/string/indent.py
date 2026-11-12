def indent(string_: str, count: int, char: str = " ") -> str:
    """
    Prefixes every non-empty line with ``count`` repetitions of ``char``.

    Empty lines are left untouched, which makes this the inverse of
    ``dedent``. A negative ``count`` is treated as 0.

    Args:
        string_: The string to indent.
        count: The number of times ``char`` is repeated per line.
        char: The indentation unit (default " ").

    Returns:
        The indented string.

    Example:
        >>> indent("line1\\nline2", 2)
        '  line1\\n  line2'
        >>> indent("a\\n\\nb", 1, "\\t")
        '\\ta\\n\\n\\tb'
    """
    prefix = char * max(0, count)
    return "\n".join(
        prefix + line if len(line) > 0 else line for line in string_.split("\n")
    )
