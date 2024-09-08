import type { SimplifiedUserAgentInfo } from "$/ua/simplifiedUserAgentInfo";

export const extractBrowserFromUserAgent = (
  ua: string,
): SimplifiedUserAgentInfo["browser"] => {
  if (/edg(e)?/i.test(ua)) {
    return "edge";
  }
  if (/chrome|crios/i.test(ua)) {
    return "chrome";
  }
  if (/firefox|fxios/i.test(ua)) {
    return "firefox";
  }
  if (/safari/i.test(ua)) {
    return "safari";
  }
  if (/msie|trident/i.test(ua)) {
    return "ie";
  }
  return "other";
};
