from urllib.parse import parse_qs, urlparse


def parse_query_string(query: str) -> dict[str, str]:
    """Parses a query string into a key-value dictionary.

    Accepts either a full URL or a raw query string
    (with or without leading "?").

    Dangerous keys (__proto__, constructor, prototype) are
    rejected to prevent prototype pollution attacks.

    :param query: The query string or URL to parse
    :return: A dictionary of key-value pairs from the query string
    """
    search_string = query
    if "://" in query:
        search_string = urlparse(query).query
    elif search_string.startswith("?"):
        search_string = search_string[1:]

    # parse_qs returns dict[str, list[str]], take the last value for each key
    parsed = parse_qs(search_string, keep_blank_values=True)
    result: dict[str, str] = {}
    for key, values in parsed.items():
        # Prevent prototype pollution by rejecting dangerous keys
        if key in ("__proto__", "constructor", "prototype"):
            continue
        result[key] = values[-1]
    return result
