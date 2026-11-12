from urllib.parse import quote_plus, urlencode


def stringify_query(params: dict[str, str]) -> str:
    """Serializes a key-value dictionary into a query string.

    The inverse of ``parse_query_string``. Keys and values are serialized as
    ``application/x-www-form-urlencoded``, the same way ``build_url``
    appends parameters: spaces become ``+`` and every character except ASCII
    letters, digits, ``*``, ``-``, ``.`` and ``_`` is percent-encoded. The
    result has no leading "?".

    :param params: A dictionary of key-value pairs to serialize
    :return: The query string, or an empty string for an empty dictionary

    :example:
        >>> stringify_query({"page": "1", "q": "search"})
        'page=1&q=search'
        >>> stringify_query({"q": "hello world", "tag": "a&b"})
        'q=hello+world&tag=a%26b'
    """
    return urlencode(params, safe="*", quote_via=quote_plus).replace("~", "%7E")
