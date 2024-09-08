import type { SimplifiedUserAgentInfo } from "$/ua/simplifiedUserAgentInfo";

export const extractDeviceFromUserAgent = (
  ua: string,
): SimplifiedUserAgentInfo["device"] => {
  if (/bot|googlebot|crawler|spider|robot|crawling/i.test(ua)) {
    return "bot";
  }
  if (
    /iphone|ipod|android.*mobile|mobile.*android|webos|blackberry|iemobile|opera mini/i.test(
      ua,
    )
  ) {
    return "mobile";
  }
  if (/ipad|android(?!.*mobile)/i.test(ua)) {
    return "tablet";
  }
  if (/windows|macintosh|linux/i.test(ua)) {
    return "desktop";
  }
  return "other";
};
