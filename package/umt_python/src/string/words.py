import re
import unicodedata


def _is_lowercase_letter(char: str) -> bool:
    """Matches the Unicode ``\\p{Ll}`` property (lowercase letter)."""
    return unicodedata.category(char) == "Ll"


def _is_uppercase_letter(char: str) -> bool:
    """Matches the Unicode ``\\p{Lu}`` property (uppercase letter)."""
    return unicodedata.category(char) == "Lu"


def _is_letter(char: str) -> bool:
    """Matches the Unicode ``\\p{L}`` property (any letter)."""
    return unicodedata.category(char).startswith("L")


def _is_number(char: str) -> bool:
    """Matches the Unicode ``\\p{N}`` property (any numeric character)."""
    return unicodedata.category(char).startswith("N")


def _add_boundaries(string_: str) -> str:
    """Inserts spaces on case boundaries, mirroring the TypeScript regex passes.

    Replicates ``([\\p{Ll}\\p{N}])(\\p{Lu})`` -> ``$1 $2`` followed by
    ``(\\p{Lu})(\\p{Lu}\\p{Ll})`` -> ``$1 $2``.
    """
    # Pass 1: lowercase letter or number followed by an uppercase letter.
    first_pass: list[str] = []
    for index, char in enumerate(string_):
        if (
            index > 0
            and _is_uppercase_letter(char)
            and (
                _is_lowercase_letter(string_[index - 1])
                or _is_number(string_[index - 1])
            )
        ):
            first_pass.append(" ")
        first_pass.append(char)
    after_first = "".join(first_pass)

    # Pass 2: uppercase letter followed by uppercase letter + lowercase letter.
    second_pass: list[str] = []
    for index, char in enumerate(after_first):
        if (
            index > 0
            and _is_uppercase_letter(after_first[index - 1])
            and _is_uppercase_letter(char)
            and index + 1 < len(after_first)
            and _is_lowercase_letter(after_first[index + 1])
        ):
            second_pass.append(" ")
        second_pass.append(char)
    return "".join(second_pass)


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

    with_boundaries = _add_boundaries(string_)

    result: list[str] = []
    current: list[str] = []
    for char in with_boundaries:
        if _is_letter(char) or _is_number(char):
            current.append(char)
        elif current:
            result.append("".join(current))
            current = []
    if current:
        result.append("".join(current))
    return result
