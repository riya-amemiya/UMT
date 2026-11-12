import re

_WORD_SEPARATOR = re.compile(r"[ \t\r\f\v]+")


def wrap_text(string_: str, width: int) -> str:
    """
    Wraps text to lines of at most ``width`` code points at word boundaries.

    Existing line breaks are kept and each line is wrapped on its own. Words
    are separated by spaces, tabs, carriage returns, form feeds and vertical
    tabs, and a word longer than ``width`` is placed on its own line without
    being broken.

    Args:
        string_: The text to wrap.
        width: The maximum line width.

    Returns:
        The wrapped text joined with ``"\\n"``.

    Example:
        >>> wrap_text("The quick brown fox", 10)
        'The quick\\nbrown fox'
        >>> wrap_text("a extraordinary b", 5)
        'a\\nextraordinary\\nb'
    """
    paragraphs: list[str] = []
    for paragraph in string_.split("\n"):
        lines: list[str] = []
        for word in _WORD_SEPARATOR.split(paragraph):
            if len(word) == 0:
                continue
            if lines and len(lines[-1]) + 1 + len(word) <= width:
                lines[-1] += " " + word
            else:
                lines.append(word)
        paragraphs.append("\n".join(lines))
    return "\n".join(paragraphs)
