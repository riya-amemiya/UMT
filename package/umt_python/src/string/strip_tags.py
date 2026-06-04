import re

_TAG_PATTERN = re.compile(r"<[^<>]*>")


def strip_tags(string_: str) -> str:
    """
    Removes HTML/XML tags from a string, keeping the text content.
    Unlike :func:`escape_html`, which escapes special characters, this deletes
    the tags entirely. The removal repeats until the string is stable so that
    nested constructs (e.g. "<sc<script>ript>") cannot survive a single pass.

    Args:
        string_: Input string.

    Returns:
        String with tags removed.

    Example:
        >>> strip_tags("<p>Hello <b>World</b></p>")
        'Hello World'
        >>> strip_tags("<sc<script>ript>")
        ''
        >>> strip_tags("plain text")
        'plain text'
    """
    result = string_
    while True:
        previous = result
        result = _TAG_PATTERN.sub("", result)
        if result == previous:
            break
    return result
