import re

_ABSOLUTE_URL_RE = re.compile(r"^[a-z][a-z\d+\-.]*:", re.IGNORECASE)


def is_absolute_url(url: str) -> bool:
    """Checks whether a URL string is absolute (RFC 3986).

    An absolute URL starts with a scheme followed by a colon,
    where the scheme begins with a letter and may contain
    letters, digits, plus, hyphen, or period.

    :param url: The URL string to check
    :return: True if the URL is absolute, False otherwise
    """
    return bool(_ABSOLUTE_URL_RE.match(url))
