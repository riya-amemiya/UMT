import re


def camel_case(string_: str) -> str:
    """
    Converts a string to camelCase.

    Args:
        string_: The string to convert.

    Returns:
        The camelCase string.

    Example:
        >>> camel_case("hello world")
        'helloWorld'
        >>> camel_case("foo-bar-baz")
        'fooBarBaz'
        >>> camel_case("FooBar")
        'fooBar'
    """
    result = re.sub(r"[^a-zA-Z0-9]+(.)", lambda m: m.group(1).upper(), string_)
    result = re.sub(r"[^a-zA-Z0-9]+$", "", result)
    if result:
        result = result[0].lower() + result[1:]
    return result
