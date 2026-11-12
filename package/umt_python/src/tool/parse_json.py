import json
from typing import Any, NoReturn

from ..object.dangerous_keys import DANGEROUS_KEYS


def _strip_dangerous_keys(parsed: dict[str, Any]) -> dict[str, Any]:
    for key in DANGEROUS_KEYS:
        parsed.pop(key, None)
    return parsed


def _reject_constant(constant: str) -> NoReturn:
    raise json.JSONDecodeError(f"Unexpected token {constant}", constant, 0)


def parse_json(json_str: str) -> Any:  # noqa: ANN401
    """
    Parses a JSON string into a Python value.

    The keys "__proto__", "constructor" and "prototype" are stripped from
    every parsed object, and the non-standard constants NaN, Infinity and
    -Infinity are rejected, matching the TypeScript implementation.

    Args:
        json_str: JSON string to parse

    Returns:
        Parsed value

    Raises:
        json.JSONDecodeError: If the JSON string is invalid

    Example:
        >>> parse_json('{"a": 1}')
        {'a': 1}
        >>> parse_json('{"__proto__": {"x": 1}, "safe": 1}')
        {'safe': 1}
    """
    return json.loads(
        json_str,
        object_hook=_strip_dangerous_keys,
        parse_constant=_reject_constant,
    )
