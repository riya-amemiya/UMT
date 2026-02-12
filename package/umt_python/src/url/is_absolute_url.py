import re


def is_absolute_url(url: str) -> bool:
    """Checks whether a URL string is absolute (RFC 3986).

    An absolute URL starts with a scheme followed by a colon,
    where the scheme begins with a letter and may contain
    letters, digits, plus, hyphen, or period.

    :param url: The URL string to check
    :return: True if the URL is absolute, False otherwise
    """
    return bool(re.match(r"^[a-z][a-z\d+\-.]*:", url, re.IGNORECASE))
