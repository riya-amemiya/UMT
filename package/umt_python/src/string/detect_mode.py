from typing import Any

# Sentinel representing JavaScript ``undefined``. Python only has ``None``
# (which maps to JavaScript ``null``), so a distinct marker is needed to tell
# an omitted argument apart from an explicit ``null`` value.
_UNDEFINED = object()


def _is_plain_object(value: Any) -> bool:  # noqa: ANN401
    """Mirrors the TypeScript ``typeof x === "object"`` non-array/Date check.

    JavaScript arrays and ``Date`` instances are excluded from named mode; the
    Python analogue is a ``dict`` (lists and ``datetime`` instances are not
    dicts, so they fall through to indexed mode).
    """
    return value is not _UNDEFINED and isinstance(value, dict)


def detect_mode(
    data_or_first_value: Any = _UNDEFINED,  # noqa: ANN401
    options_or_second_value: Any = _UNDEFINED,  # noqa: ANN401
    rest_values: list[Any] | None = None,
) -> dict[str, Any]:
    """
    Detects whether formatString should use indexed or named mode based on
    arguments.

    Named mode: the first argument is a non-array mapping. Indexed mode: the
    arguments are treated as array values. Omit ``data_or_first_value`` /
    ``options_or_second_value`` to represent JavaScript ``undefined``; pass an
    explicit ``None`` for JavaScript ``null``.

    Args:
        data_or_first_value: First argument (mapping for named mode, value for
            indexed mode).
        options_or_second_value: Second argument (options for named mode, value
            for indexed mode).
        rest_values: Remaining arguments for indexed mode.

    Returns:
        Mapping with ``"data"`` and ``"options"`` keys for formatting.

    Example:
        >>> detect_mode({"name": "Alice"})
        {'data': {'name': 'Alice'}, 'options': {}}
        >>> detect_mode("first", "second", ["third"])
        {'data': ['first', 'second', 'third'], 'options': {}}
    """
    rest = rest_values if rest_values is not None else []

    is_first_argument_object = _is_plain_object(data_or_first_value)

    is_second_argument_options = (
        _is_plain_object(options_or_second_value)
        and "formatters" in options_or_second_value
    )

    if (
        is_first_argument_object
        and options_or_second_value is _UNDEFINED
        and len(rest) == 0
    ):
        return {"data": data_or_first_value, "options": {}}

    if is_first_argument_object and is_second_argument_options and len(rest) == 0:
        return {"data": data_or_first_value, "options": options_or_second_value}

    all_values: list[Any] = []
    if data_or_first_value is not _UNDEFINED:
        all_values.append(data_or_first_value)
    if options_or_second_value is not _UNDEFINED:
        all_values.append(options_or_second_value)
    all_values.extend(rest)

    return {"data": all_values, "options": {}}
