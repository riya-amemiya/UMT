import { extractBrowserFromUserAgent } from "./extractBrowserFromUserAgent";
import { extractDeviceFromUserAgent } from "./extractDeviceFromUserAgent";
import { extractOSFromUserAgent } from "./extractOSFromUserAgent";

import type { SimplifiedUserAgentInfo } from "$/ua/simplifiedUserAgentInfo";

export const parseUserAgent = (userAgent: string): SimplifiedUserAgentInfo => {
  const ua = userAgent.toLowerCase();

  return {
    os: extractOSFromUserAgent(ua),
    browser: extractBrowserFromUserAgent(ua),
    device: extractDeviceFromUserAgent(ua),
  };
};
