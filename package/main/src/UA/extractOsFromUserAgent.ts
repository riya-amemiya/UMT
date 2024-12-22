import type { SimplifiedUserAgentInfoOs } from "$/ua/simplifiedUserAgentInfoOs";

export const extractOsFromUserAgent = (
  ua: string,
): SimplifiedUserAgentInfoOs => {
  if (/iphone|ipad|ipod/i.test(ua)) {
    return "ios";
  }
  if (/android/i.test(ua)) {
    return "android";
  }
  if (/mac os x/i.test(ua)) {
    return "macos";
  }
  if (/windows|win32/i.test(ua)) {
    return "windows";
  }
  if (/linux/i.test(ua)) {
    return "linux";
  }
  return "other";
};
