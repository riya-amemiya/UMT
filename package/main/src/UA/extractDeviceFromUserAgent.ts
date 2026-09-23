import type { SimplifiedUserAgentInfoDevice } from "$/ua/simplifiedUserAgentInfoDevice";

/**
 * Extracts device type information from a User-Agent string
 *
 * @param ua - The User-Agent string to analyze
 * @returns The detected device type ("bot", "mobile", "tablet", "desktop", or "other")
 *
 * @example
 * ```ts
 * const device = extractDeviceFromUserAgent(navigator.userAgent);
 * // device: "desktop"
 * ```
 */
export const extractDeviceFromUserAgent = (
  ua: string,
): SimplifiedUserAgentInfoDevice => {
  if (/bot|googlebot|crawler|spider|robot|crawling/i.test(ua)) {
    return "bot";
  }

  if (/iphone|ipod|webos|blackberry|iemobile|opera mini/i.test(ua)) {
    return "mobile";
  }

  if (/android/i.test(ua)) {
    return /mobile/i.test(ua) ? "mobile" : "tablet";
  }

  if (/ipad|android(?!.*mobile)/i.test(ua)) {
    return "tablet";
  }

  return /windows|macintosh|linux/i.test(ua) ? "desktop" : "other";
};
