from typing import Literal, Dict, List


DayLang = Literal["de", "ko", "en", "ja", "fr"]


DAY_LIST: Dict[DayLang, List[str]] = {
    "de": ["So", "Mo", "Di", "Mi", "Do", "Fr", "Sa"],
    "ko": ["일", "월", "화", "수", "목", "금", "토"],
    "en": ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"],
    "ja": ["日", "月", "火", "水", "木", "金", "土"],
    "fr": ["Dim", "Lun", "Mar", "Mer", "Jeu", "Ven", "Sam"],
}


def get_day(day: int, lang: DayLang = "ja") -> str:
    """
    Convert a number to a day of the week in the specified language.

    Args:
        day: Number representing the day (0-6, where 0 is Sunday)
        lang: Language code ('de', 'ko', 'en', 'ja', 'fr')

    Returns:
        Day of the week string in the specified language

    Example:
        >>> get_day(0)
        '日'
        >>> get_day(0, "en")
        'Sun'
        >>> get_day(1, "fr")
        'Lun'
    """
    if day < 0 or day > 6:
        return DAY_LIST[lang][0]
    return DAY_LIST[lang][day]
