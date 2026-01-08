import re
from typing import Literal


BrowserType = Literal["edge", "chrome", "firefox", "safari", "ie", "other"]


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
    if re.search(r"edg(e)?", ua, re.IGNORECASE):
        return "edge"
    if re.search(r"msie|trident", ua, re.IGNORECASE):
        return "ie"
    if re.search(r"firefox|fxios", ua, re.IGNORECASE):
        return "firefox"
    if re.search(r"opr/", ua, re.IGNORECASE):
        return "other"
    if re.search(r"chrome|crios", ua, re.IGNORECASE):
        return "chrome"
    if re.search(r"safari", ua, re.IGNORECASE):
        return "safari"
    return "other"
