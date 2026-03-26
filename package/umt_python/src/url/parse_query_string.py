from urllib.parse import parse_qs, urlparse

# Keys rejected to prevent prototype pollution in downstream consumers
_DANGEROUS_KEYS = frozenset({"__proto__", "constructor", "prototype"})


def parse_query_string(query: str) -> dict[str, str]:
    """Parses a query string into a key-value dictionary.

    Accepts either a full URL or a raw query string
    (with or without leading ``?``).

    Keys that could cause prototype pollution in downstream
    JavaScript consumers (``__proto__``, ``constructor``,
    ``prototype``) are silently dropped for defence-in-depth.

    Args:
        query: The query string or URL to parse.

    Returns:
        A dictionary of key-value pairs from the query string.

    Examples:
        >>> parse_query_string("?page=1&q=search")
        {'page': '1', 'q': 'search'}

        >>> parse_query_string("foo=bar&baz=qux")
        {'foo': 'bar', 'baz': 'qux'}

        >>> parse_query_string("https://example.com?a=1&b=2")
        {'a': '1', 'b': '2'}
    """
    if "://" in query:
        search_string = urlparse(query).query
    elif query.startswith("?"):
        search_string = query[1:]
    else:
        search_string = query

    if not search_string:
        return {}

    parsed = parse_qs(search_string, keep_blank_values=True)
    return {
        key: values[0] for key, values in parsed.items() if key not in _DANGEROUS_KEYS
    }
