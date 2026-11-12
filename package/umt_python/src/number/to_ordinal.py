import math

from ..math.number_to_js_string import number_to_js_string


def to_ordinal(value: int | float) -> str:
    """
    Converts a number to its English ordinal string representation.

    Handles the special cases for 11th, 12th, and 13th
    (which end in "th" despite their last digit).

    :param value: The number to convert to ordinal form
    :return: The ordinal string (e.g., "1st", "2nd", "3rd", "11th")

    example:
    to_ordinal(1)   # "1st"
    to_ordinal(2)   # "2nd"
    to_ordinal(3)   # "3rd"
    to_ordinal(11)  # "11th"
    to_ordinal(21)  # "21st"
    to_ordinal(112) # "112th"
    """
    text = number_to_js_string(value) if isinstance(value, float) else str(value)
    if not math.isfinite(value):
        return f"{text}th"

    module100 = math.fmod(value, 100)
    if 11 <= module100 <= 13:
        return f"{text}th"

    module10 = math.fmod(value, 10)
    if module10 == 1:
        return f"{text}st"
    if module10 == 2:
        return f"{text}nd"
    if module10 == 3:
        return f"{text}rd"

    return f"{text}th"
