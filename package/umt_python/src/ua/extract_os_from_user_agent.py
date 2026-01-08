import re
from typing import Literal


OsType = Literal["ios", "android", "macos", "windows", "linux", "other"]


def extract_os_from_user_agent(ua: str) -> OsType:
    """
    Extracts operating system information from a User-Agent string.

    Args:
        ua: The User-Agent string to analyze

    Returns:
        The detected operating system ("ios", "android", "macos", "windows", "linux", or "other")

    Example:
        >>> extract_os_from_user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7)")
        'macos'
    """
    if re.search(r"iphone|ipad|ipod", ua, re.IGNORECASE):
        return "ios"
    if re.search(r"android", ua, re.IGNORECASE):
        return "android"
    if re.search(r"mac os x", ua, re.IGNORECASE):
        return "macos"
    if re.search(r"windows|win32", ua, re.IGNORECASE):
        return "windows"
    if re.search(r"linux", ua, re.IGNORECASE):
        return "linux"
    return "other"
