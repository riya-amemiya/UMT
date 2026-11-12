from urllib.parse import parse_qsl, urlparse


def parse_query_string(query: str) -> dict[str, str]:
    """Parses a query string into a key-value dictionary.

    Accepts either a full URL or a raw query string
    (with or without leading "?").

    When a key appears more than once, the last value wins. No key is
    filtered out.

    :param query: The query string or URL to parse
    :return: A dictionary of key-value pairs from the query string
    :raises ValueError: If the input contains "://" but has no URL scheme
    """
    search_string = query
    if "://" in query:
        parsed = urlparse(query)
        if not parsed.scheme:
            raise ValueError("Invalid URL")
        search_string = parsed.query
    elif search_string.startswith("?"):
        search_string = search_string[1:]

    return dict(parse_qsl(search_string, keep_blank_values=True))
