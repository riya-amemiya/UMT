import type { SimplifiedUserAgentInfoBrowser } from "$/ua/simplifiedUserAgentInfoBrowser";

/**
 * Extracts browser information from a User-Agent string
 *
 * @param ua - The User-Agent string to analyze
 * @returns The detected browser type ("edge", "chrome", "firefox", "safari", "ie", or "other")
 *
 * @example
 * ```ts
 * const browser = extractBrowserFromUserAgent(navigator.userAgent);
 * // browser: "chrome"
 * ```
 */
export const extractBrowserFromUserAgent = (
  ua: string,
): SimplifiedUserAgentInfoBrowser => {
  if (/edg(e)?/i.test(ua)) {
    return "edge";
  }
  if (/msie|trident/i.test(ua)) {
    return "ie";
  }
  if (/firefox|fxios/i.test(ua)) {
    return "firefox";
  }
  // Opera uses Chromium, so check for OPR before Chrome
  if (/opr\//i.test(ua)) {
    return "other";
  }
  if (/chrome|crios/i.test(ua)) {
    return "chrome";
  }
  // Safari check should be last as Chrome/Firefox on iOS also include Safari in UA
  if (/safari/i.test(ua)) {
    return "safari";
  }
  return "other";
};
