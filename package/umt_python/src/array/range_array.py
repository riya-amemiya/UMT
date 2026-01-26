def range_array(
    start: int | float,
    end: int | float | None = None,
    step: int | float = 1,
) -> list[int | float]:
    """
    Generates an array of sequential numbers.

    Args:
        start: Starting number.
        end: Ending number (if omitted, generates array from 0 to start).
        step: Step value (defaults to 1).

    Returns:
        Array of sequential numbers.

    Example:
        >>> range_array(5)
        [0, 1, 2, 3, 4]
        >>> range_array(2, 10, 2)
        [2, 4, 6, 8]
    """
    result: list[int | float] = []

    if step == 0:
        return result

    actual_start = 0 if end is None else start
    actual_end = start if end is None else end

    if (step > 0 and actual_start >= actual_end) or (
        step < 0 and actual_start <= actual_end
    ):
        return result

    if step > 0:
        index = actual_start
        while index < actual_end:
            rounded_value = round(index * 1e10) / 1e10
            result.append(rounded_value)
            index += step
    else:
        index = actual_start
        while index > actual_end:
            rounded_value = round(index * 1e10) / 1e10
            result.append(rounded_value)
            index += step

    return result
