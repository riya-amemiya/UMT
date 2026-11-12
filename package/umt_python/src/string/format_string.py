import re
from typing import Any

from ..math.js_string_to_number import js_string_to_number
from .apply_formatter import apply_formatter, js_string
from .default_formatters import default_formatters
from .detect_mode import detect_mode
from .get_value import get_value


def _indexed_value(data: list[Any], path: str) -> Any:  # noqa: ANN401
    index = js_string_to_number(path)
    if not index.is_integer() or index < 0 or index >= len(data):
        return None
    return data[int(index)]


def format_string(template: str, *values: Any) -> str:  # noqa: ANN401
    """
    Replaces placeholders in a template string with specified values.

    Supports two modes:

    1. Indexed mode: the values are referenced as ``{0}``, ``{1}``, ...
    2. Named mode: a single mapping (optionally followed by an options mapping
       holding custom ``formatters``) is referenced by path, with nested paths
       (``{user.name}``), array access (``{items[0]}``, ``{items[-1]}``),
       default values (``{name|Guest}``), formatters
       (``{price:number(en-US,2,2)}``) and escaped braces (``{{`` / ``}}``).

    Args:
        template: Template string containing placeholders.
        *values: Values to replace the placeholders in the template, or a data
            mapping and an optional options mapping.

    Returns:
        String with placeholders replaced with values.

    Example:
        >>> format_string("Hello, {0}!", "World")
        'Hello, World!'
        >>> format_string("The sum of {0} and {1} is {2}.", 1, 2, 3)
        'The sum of 1 and 2 is 3.'
        >>> format_string("Not enough values: {0} {1}", "val1")
        'Not enough values: val1 {1}'
        >>> format_string("Hello, {name|Guest}!", {"name": "Alice"})
        'Hello, Alice!'
        >>> format_string("{user.name:upper}", {"user": {"name": "bob"}})
        'BOB'
    """
    escaped_template = template.replace("{{", "\x00").replace("}}", "\x01")

    mode = detect_mode(*values[:2], rest_values=list(values[2:]))
    data = mode["data"]
    formatters = {**default_formatters, **mode["options"].get("formatters", {})}

    def replace(match: re.Match[str]) -> str:
        parts = [part.strip() for part in match.group(1).split("|")]
        default_value = parts[1] if len(parts) > 1 else None
        path, *formatter_parts = parts[0].split(":")
        formatter_string = ":".join(formatter_parts)

        value = (
            _indexed_value(data, path)
            if isinstance(data, list)
            else get_value(data, path)
        )

        if value is None:
            if default_value is None:
                return match.group(0)
            value = default_value

        return (
            apply_formatter(value, formatter_string, formatters)
            if formatter_string
            else js_string(value)
        )

    result = re.sub(r"{([^}]+)}", replace, escaped_template)

    return result.replace("\x00", "{").replace("\x01", "}")
