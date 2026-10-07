import re
from typing import Literal

OsType = Literal["ios", "android", "macos", "windows", "linux", "other"]

_IOS_RE = re.compile(r"iphone|ipad|ipod", re.IGNORECASE)
_ANDROID_RE = re.compile(r"android", re.IGNORECASE)
_MACOS_RE = re.compile(r"mac os x", re.IGNORECASE)
_WINDOWS_RE = re.compile(r"windows|win32", re.IGNORECASE)
_LINUX_RE = re.compile(r"linux", re.IGNORECASE)


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
    if _IOS_RE.search(ua):
        return "ios"
    if _ANDROID_RE.search(ua):
        return "android"
    if _MACOS_RE.search(ua):
        return "macos"
    if _WINDOWS_RE.search(ua):
        return "windows"
    if _LINUX_RE.search(ua):
        return "linux"
    return "other"
