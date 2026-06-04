import math


def dedent(string_: str) -> str:
    """
    Removes the minimum common leading whitespace from each line of the input.

    Only spaces and tabs count toward a line's indentation, and fully blank
    lines are ignored when computing the minimum indent. When no line carries
    indentation, the input is returned unchanged.

    Args:
        string_: Input string.

    Returns:
        Dedented string.

    Example:
        >>> dedent("    line1\\n      line2\\n    line3")
        'line1\\n  line2\\nline3'
        >>> dedent("    a\\n\\n    b")
        'a\\n\\nb'
        >>> dedent("no indent")
        'no indent'
        >>> dedent("\\n  \\n")
        '\\n  \\n'
    """
    lines = string_.split("\n")
    min_indent = math.inf
    for line in lines:
        if len(line.strip()) == 0:
            continue
        indent = 0
        while indent < len(line) and line[indent] in (" ", "\t"):
            indent += 1
        min_indent = min(min_indent, indent)
    if not math.isfinite(min_indent):
        return string_
    min_indent = int(min_indent)
    return "\n".join(line[min_indent:] for line in lines)
