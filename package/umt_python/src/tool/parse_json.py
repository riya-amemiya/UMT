import json
from typing import Any, TypeVar

T = TypeVar("T")


def parse_json(json_str: str) -> Any:
    """
    Parses a JSON string into a Python value.

    Args:
        json_str: JSON string to parse

    Returns:
        Parsed value

    Raises:
        json.JSONDecodeError: If the JSON string is invalid

    Example:
        >>> parse_json('{"a": 1}')
        {'a': 1}
    """
    return json.loads(json_str)
