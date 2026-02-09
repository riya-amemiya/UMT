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
    module100 = value % 100
    if 11 <= module100 <= 13:
        return f"{value}th"

    module10 = value % 10
    if module10 == 1:
        return f"{value}st"
    if module10 == 2:
        return f"{value}nd"
    if module10 == 3:
        return f"{value}rd"

    return f"{value}th"
