import re
from typing import Literal

BrowserType = Literal["edge", "chrome", "firefox", "safari", "ie", "other"]

_EDGE_RE = re.compile(r"edg(e)?", re.IGNORECASE)
_IE_RE = re.compile(r"msie|trident", re.IGNORECASE)
_FIREFOX_RE = re.compile(r"firefox|fxios", re.IGNORECASE)
_OPERA_RE = re.compile(r"opr/", re.IGNORECASE)
_CHROME_RE = re.compile(r"chrome|crios", re.IGNORECASE)
_SAFARI_RE = re.compile(r"safari", re.IGNORECASE)


def extract_browser_from_user_agent(ua: str) -> BrowserType:
    """
    Extracts browser information from a User-Agent string.

    Args:
        ua: The User-Agent string to analyze

    Returns:
        The detected browser type ("edge", "chrome", "firefox", "safari", "ie", or "other")

    Example:
        >>> extract_browser_from_user_agent("Mozilla/5.0 Chrome/91.0")
        'chrome'
    """
    if _EDGE_RE.search(ua):
        return "edge"
    if _IE_RE.search(ua):
        return "ie"
    if _FIREFOX_RE.search(ua):
        return "firefox"
    if _OPERA_RE.search(ua):
        return "other"
    if _CHROME_RE.search(ua):
        return "chrome"
    if _SAFARI_RE.search(ua):
        return "safari"
    return "other"
