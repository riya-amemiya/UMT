import re
from typing import Literal


DeviceType = Literal["bot", "mobile", "tablet", "desktop", "other"]


def extract_device_from_user_agent(ua: str) -> DeviceType:
    """
    Extracts device type information from a User-Agent string.

    Args:
        ua: The User-Agent string to analyze

    Returns:
        The detected device type ("bot", "mobile", "tablet", "desktop", or "other")

    Example:
        >>> extract_device_from_user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        'desktop'
    """
    if re.search(r"bot|googlebot|crawler|spider|robot|crawling", ua, re.IGNORECASE):
        return "bot"

    if re.search(
        r"iphone|ipod|webos|blackberry|iemobile|opera mini", ua, re.IGNORECASE
    ):
        return "mobile"

    if re.search(r"android", ua, re.IGNORECASE):
        if re.search(r"mobile", ua, re.IGNORECASE):
            return "mobile"
        return "tablet"

    if re.search(r"ipad|android(?!.*mobile)", ua, re.IGNORECASE):
        return "tablet"

    if re.search(r"windows|macintosh|linux", ua, re.IGNORECASE):
        return "desktop"

    return "other"
