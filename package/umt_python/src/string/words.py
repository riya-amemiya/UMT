import re
import unicodedata


def words(string_: str, pattern: re.Pattern[str] | None = None) -> list[str]:
    """
    Splits a string into words on case boundaries and non-alphanumeric
    separators. Uses Unicode classification so CJK and other letters are
    preserved.

    Args:
        string_: Input string.
        pattern: Custom compiled matching pattern; defaults to a Unicode word
            matcher.

    Returns:
        List of words.

    Example:
        >>> words("helloWorld foo-bar")
        ['hello', 'World', 'foo', 'bar']
        >>> words("XMLHttpRequest")
        ['XML', 'Http', 'Request']
        >>> words("")
        []
    """
    if pattern is not None:
        return pattern.findall(string_)

    categories = [unicodedata.category(char) for char in string_]
    last_index = len(categories) - 1

    result: list[str] = []
    start = -1
    for index, category in enumerate(categories):
        if category[0] not in "LN":
            if start >= 0:
                result.append(string_[start:index])
                start = -1
            continue
        if start < 0:
            start = index
            continue
        if category != "Lu":
            continue
        previous = categories[index - 1]
        if (
            previous == "Ll"
            or previous[0] == "N"
            or (
                previous == "Lu"
                and index < last_index
                and categories[index + 1] == "Ll"
            )
        ):
            result.append(string_[start:index])
            start = index
    if start >= 0:
        result.append(string_[start:])
    return result
