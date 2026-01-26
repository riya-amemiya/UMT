import re

HTML_ESCAPE_MAP: dict[str, str] = {
    "&": "&amp;",
    "<": "&lt;",
    ">": "&gt;",
    '"': "&quot;",
    "'": "&#39;",
}


def escape_html(string_: str) -> str:
    """
    Escapes HTML special characters in a string.

    Args:
        string_: The string to escape.

    Returns:
        The escaped string.

    Example:
        >>> escape_html("<script>alert('XSS')</script>")
        '&lt;script&gt;alert(&#39;XSS&#39;)&lt;/script&gt;'
        >>> escape_html('Tom & Jerry')
        'Tom &amp; Jerry'
        >>> escape_html('"Hello"')
        '&quot;Hello&quot;'
    """
    return re.sub(r"[&<>\"']", lambda m: HTML_ESCAPE_MAP[m.group(0)], string_)
