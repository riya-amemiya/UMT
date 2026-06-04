import re
from collections.abc import Callable
from typing import Any

from ..math.number_to_js_string import number_to_js_string

# A formatter receives the raw value plus zero or more string arguments and
# returns the formatted string. Mirrors the TypeScript ``Formatter`` type.
Formatter = Callable[..., str]

# JavaScript-style formatter syntax: ``name`` or ``name(arguments)``. ``\w`` in
# JavaScript is ASCII only, so the name class is restricted to ASCII word
# characters.
_FORMATTER_PATTERN = re.compile(r"^([A-Za-z0-9_]+)(?:\(([^)]*)\))?$")


def js_string(value: Any) -> str:  # noqa: ANN401
    """Converts a value to a string following JavaScript ``String()`` rules.

    JavaScript distinguishes ``null`` and ``undefined``; Python only has
    ``None``, which is rendered as ``"null"``.

    Args:
        value: Value to stringify.

    Returns:
        The JavaScript-equivalent string representation.
    """
    if value is None:
        return "null"
    if isinstance(value, bool):
        return "true" if value else "false"
    if isinstance(value, (int, float)):
        return number_to_js_string(float(value))
    if isinstance(value, str):
        return value
    if isinstance(value, list):
        return ",".join("" if item is None else js_string(item) for item in value)
    return "[object Object]"


def apply_formatter(
    value: Any,  # noqa: ANN401
    formatter_string: str,
    formatters: dict[str, Formatter],
) -> str:
    """
    Applies a formatter function to a value with optional arguments.

    Parses formatter syntax like "upper", "currency(ja-JP,JPY)", "pad(4,0)" and
    applies the corresponding formatter function with parsed arguments.

    Args:
        value: The value to format.
        formatter_string: Formatter name with optional arguments (e.g.
            "upper", "currency(ja-JP,JPY)").
        formatters: Available formatter functions keyed by name.

    Returns:
        Formatted string, or the original string value if the formatter is not
        found/invalid.

    Example:
        >>> apply_formatter("hello", "upper", {"upper": lambda v: str(v).upper()})
        'HELLO'
        >>> apply_formatter(
        ...     42,
        ...     "pad(4,0)",
        ...     {"pad": lambda v, length, char: str(v).rjust(int(length), char)},
        ... )
        '0042'
        >>> apply_formatter("test", "invalid!@#", {})
        'test'
    """
    match = _FORMATTER_PATTERN.match(formatter_string)
    if not match:
        return js_string(value)

    formatter_name, arguments_string = match.group(1), match.group(2)
    formatter = formatters.get(formatter_name)

    if not formatter:
        return js_string(value)

    arguments = _parse_arguments(arguments_string) if arguments_string else []

    return formatter(value, *arguments)


def _parse_arguments(arguments_string: str) -> list[str]:
    """Parses comma-separated arguments while preserving quoted strings.

    Args:
        arguments_string: String containing comma-separated arguments.

    Returns:
        List of parsed arguments.
    """
    arguments: list[str] = []
    chars: list[str] = []
    in_quotes = False
    quote_char = ""

    for char in arguments_string:
        if not in_quotes and char in ('"', "'"):
            in_quotes = True
            quote_char = char
            continue

        if in_quotes and char == quote_char:
            in_quotes = False
            quote_char = ""
            continue

        if not in_quotes and char == ",":
            trimmed = "".join(chars).strip()
            arguments.append(" " if trimmed == "" else trimmed)
            chars.clear()
            continue

        chars.append(char)

    trimmed = "".join(chars).strip()
    arguments.append(" " if trimmed == "" else trimmed)

    return arguments
