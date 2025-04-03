import { extractBrowserFromUserAgent } from "./extractBrowserFromUserAgent";
import { extractDeviceFromUserAgent } from "./extractDeviceFromUserAgent";
import { extractOsFromUserAgent } from "./extractOsFromUserAgent";

import type { SimplifiedUserAgentInfo } from "$/ua/simplifiedUserAgentInfo";

/**
 * Parse a User-Agent string to extract browser, device, and OS information
 *
 * @param userAgent - The complete User-Agent string to analyze
 * @returns An object containing the detected browser, device, and operating system
 *
 * @example
 * ```ts
 * const info = parseUserAgent(navigator.userAgent);
 * // info: {
 * //   browser: "chrome",
 * //   device: "desktop",
 * //   os: "macos"
 * // }
 * ```
 */
export const parseUserAgent = (userAgent: string): SimplifiedUserAgentInfo => {
  const ua = userAgent.toLowerCase();

  return {
    os: extractOsFromUserAgent(ua),
    browser: extractBrowserFromUserAgent(ua),
    device: extractDeviceFromUserAgent(ua),
  };
};
