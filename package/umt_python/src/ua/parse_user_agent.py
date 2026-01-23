from typing import TypedDict

from .extract_browser_from_user_agent import (
    BrowserType,
    extract_browser_from_user_agent,
)
from .extract_device_from_user_agent import DeviceType, extract_device_from_user_agent
from .extract_os_from_user_agent import OsType, extract_os_from_user_agent


class UserAgentInfo(TypedDict):
    browser: BrowserType
    device: DeviceType
    os: OsType


def parse_user_agent(user_agent: str) -> UserAgentInfo:
    """
    Parse a User-Agent string to extract browser, device, and OS information.

    Args:
        user_agent: The complete User-Agent string to analyze

    Returns:
        A dictionary containing the detected browser, device, and operating system

    Example:
        >>> parse_user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) Chrome/91.0")
        {'browser': 'chrome', 'device': 'desktop', 'os': 'macos'}
    """
    ua = user_agent.lower()

    return {
        "os": extract_os_from_user_agent(ua),
        "browser": extract_browser_from_user_agent(ua),
        "device": extract_device_from_user_agent(ua),
    }
