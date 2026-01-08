def is_leap_year(year: int) -> bool:
    """
    Determine if a given year is a leap year.

    Args:
        year: The year to check

    Returns:
        True if the year is a leap year, False otherwise

    Example:
        >>> is_leap_year(2000)
        True
        >>> is_leap_year(2020)
        True
        >>> is_leap_year(2100)
        False
        >>> is_leap_year(2023)
        False
    """
    if not isinstance(year, int) or year != int(year):
        return False

    return (year % 4 == 0 and year % 100 != 0) or year % 400 == 0
