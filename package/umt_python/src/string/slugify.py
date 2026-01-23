import re
import unicodedata


def slugify(string_: str) -> str:
    """
    Convert a string to a URL-friendly slug.

    Args:
        string_: The string to convert.

    Returns:
        The slugified string.

    Example:
        >>> slugify("Hello World!")
        'hello-world'
        >>> slugify("This is a Test")
        'this-is-a-test'
        >>> slugify("Café")
        'cafe'
    """
    result = unicodedata.normalize("NFD", string_)
    result = re.sub(r"[\u0300-\u036f]", "", result)
    result = result.lower()
    result = re.sub(r"[^\w\s-]", "-", result)
    result = re.sub(r"\s+", "-", result)
    result = re.sub(r"_+", "-", result)
    result = re.sub(r"-+", "-", result)
    return re.sub(r"^-+|-+$", "", result)
