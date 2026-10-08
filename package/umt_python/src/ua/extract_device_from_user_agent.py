import re
from typing import Literal

DeviceType = Literal["bot", "mobile", "tablet", "desktop", "other"]

_BOT_RE = re.compile(r"bot|googlebot|crawler|spider|robot|crawling", re.IGNORECASE)
_MOBILE_RE = re.compile(
    r"iphone|ipod|webos|blackberry|iemobile|opera mini", re.IGNORECASE
)
_ANDROID_RE = re.compile(r"android", re.IGNORECASE)
_MOBILE_CHECK_RE = re.compile(r"mobile", re.IGNORECASE)
_TABLET_RE = re.compile(r"ipad|android(?!.*mobile)", re.IGNORECASE)
_DESKTOP_RE = re.compile(r"windows|macintosh|linux", re.IGNORECASE)


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
    if _BOT_RE.search(ua):
        return "bot"

    if _MOBILE_RE.search(ua):
        return "mobile"

    if _ANDROID_RE.search(ua):
        if _MOBILE_CHECK_RE.search(ua):
            return "mobile"
        return "tablet"

    if _TABLET_RE.search(ua):
        return "tablet"

    if _DESKTOP_RE.search(ua):
        return "desktop"

    return "other"
