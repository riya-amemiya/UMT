from urllib.parse import urlencode, urlparse, urlunparse


def build_url(base: str, params: dict[str, str] | None = None) -> str:
    """Builds a URL with query parameters appended.

    Dangerous keys (__proto__, constructor, prototype) are
    rejected to prevent prototype pollution attacks.

    :param base: The base URL string
    :param params: A dictionary of key-value pairs to append as query parameters
    :return: The complete URL string with query parameters

    :example:
        >>> build_url("https://example.com", {"page": "1", "q": "search"})
        'https://example.com/?page=1&q=search'
        >>> build_url("https://example.com/path", {"foo": "bar"})
        'https://example.com/path?foo=bar'
        >>> build_url("https://example.com/path")
        'https://example.com/path'
    """
    if params is None:
        params = {}

    parsed = urlparse(base)

    # Normalize: ensure path is at least "/" when a netloc is present
    # (matching URL constructor behavior in JavaScript)
    path = parsed.path or "/"

    # Filter out dangerous keys
    safe_params = {
        k: v
        for k, v in params.items()
        if k not in ("__proto__", "constructor", "prototype")
    }

    # Combine existing query string with new params
    existing_query = parsed.query
    new_query = urlencode(safe_params)

    if existing_query and new_query:
        combined_query = f"{existing_query}&{new_query}"
    elif new_query:
        combined_query = new_query
    else:
        combined_query = existing_query

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
