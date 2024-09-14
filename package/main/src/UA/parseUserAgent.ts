import { extractBrowserFromUserAgent } from "./extractBrowserFromUserAgent";
import { extractDeviceFromUserAgent } from "./extractDeviceFromUserAgent";
import { extractOsFromUserAgent } from "./extractOsFromUserAgent";

import type { SimplifiedUserAgentInfo } from "$/ua/simplifiedUserAgentInfo";

export const parseUserAgent = (userAgent: string): SimplifiedUserAgentInfo => {
  const ua = userAgent.toLowerCase();

  return {
    os: extractOsFromUserAgent(ua),
    browser: extractBrowserFromUserAgent(ua),
    device: extractDeviceFromUserAgent(ua),
  };
};
