from urllib.parse import parse_qsl, quote_plus, urlencode, urlparse, urlunparse

_SPECIAL_SCHEMES = frozenset({"http", "https", "ws", "wss", "ftp", "file"})


def build_url(base: str, params: dict[str, str] | None = None) -> str:
    """Builds a URL with query parameters appended.

    Keys and values are serialized as ``application/x-www-form-urlencoded``,
    the same way JavaScript ``URLSearchParams`` does: spaces become ``+`` and
    every character except ASCII letters, digits, ``*``, ``-``, ``.`` and
    ``_`` is percent-encoded. No key is filtered out. When parameters are
    appended, the existing query string of the base URL is serialized again
    the same way.

    :param base: The base URL string
    :param params: A dictionary of key-value pairs to append as query parameters
    :return: The complete URL string with query parameters
    :raises ValueError: If the base URL has no scheme

    :example:
        >>> build_url("https://example.com", {"page": "1", "q": "search"})
        'https://example.com/?page=1&q=search'
        >>> build_url("https://example.com/path", {"foo": "bar"})
        'https://example.com/path?foo=bar'
        >>> build_url("https://example.com/path")
        'https://example.com/path'
    """
    parsed = urlparse(base)
    if not parsed.scheme:
        raise ValueError("Invalid URL")

    path = parsed.path
    if not path and parsed.netloc and parsed.scheme in _SPECIAL_SCHEMES:
        path = "/"

    combined_query = parsed.query
    if params:
        combined_query = urlencode(
            [*parse_qsl(parsed.query, keep_blank_values=True), *params.items()],
            safe="*",
            quote_via=quote_plus,
        ).replace("~", "%7E")

    return urlunparse(
        (
            parsed.scheme,
            parsed.netloc,
            path,
            parsed.params,
            combined_query,
            parsed.fragment,
        )
    )
