import { extractDeviceFromUserAgent } from "@/UA/extractDeviceFromUserAgent";

describe("extractDeviceFromUserAgent", () => {
  it("should detect bots and crawlers", () => {
    const googlebot =
      "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)";
    const bingbot =
      "Mozilla/5.0 (compatible; bingbot/2.0; +http://www.bing.com/bingbot.htm)";
    const crawler =
      "Mozilla/5.0 (compatible; YandexBot/3.0; +http://yandex.com/bots)";

    expect(extractDeviceFromUserAgent(googlebot)).toBe("bot");
    expect(extractDeviceFromUserAgent(bingbot)).toBe("bot");
    expect(extractDeviceFromUserAgent(crawler)).toBe("bot");
  });

  it("should detect mobile devices", () => {
    const iphone =
      "Mozilla/5.0 (iPhone; CPU iPhone OS 14_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0.3 Mobile/15E148 Safari/604.1";
    const androidMobile =
      "Mozilla/5.0 (Linux; Android 10; SM-A505FN) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.105 Mobile Safari/537.36";
    const blackberry =
      "Mozilla/5.0 (BlackBerry; U; BlackBerry 9900; en) AppleWebKit/534.11+ (KHTML, like Gecko) Version/7.1.0.346 Mobile Safari/534.11+";
    const operaMini =
      "Opera/9.80 (J2ME/MIDP; Opera Mini/5.1.21214/28.2725; U; ru) Presto/2.8.119 Version/11.10";

    expect(extractDeviceFromUserAgent(iphone)).toBe("mobile");
    expect(extractDeviceFromUserAgent(androidMobile)).toBe("mobile");
    expect(extractDeviceFromUserAgent(blackberry)).toBe("mobile");
    expect(extractDeviceFromUserAgent(operaMini)).toBe("mobile");
  });

  it("should detect tablets", () => {
    const ipad =
      "Mozilla/5.0 (iPad; CPU OS 14_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0 Mobile/15E148 Safari/604.1";
    const androidTablet =
      "Mozilla/5.0 (Linux; Android 11; SM-T870) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.105 Safari/537.36";
    const androidNonMobile =
      "Mozilla/5.0 (Linux; Android 11; SAMSUNG-SM-T377A Build/NMF26X) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.105 Safari/537.36";

    expect(extractDeviceFromUserAgent(ipad)).toBe("tablet");
    expect(extractDeviceFromUserAgent(androidTablet)).toBe("tablet");
    expect(extractDeviceFromUserAgent(androidNonMobile)).toBe("tablet");
  });

  it("should detect desktop devices", () => {
    const windows =
      "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36";
    const mac =
      "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.1 Safari/605.1.15";
    const linux =
      "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36";

    expect(extractDeviceFromUserAgent(windows)).toBe("desktop");
    expect(extractDeviceFromUserAgent(mac)).toBe("desktop");
    expect(extractDeviceFromUserAgent(linux)).toBe("desktop");
  });

  it("should handle unknown devices", () => {
    const empty = "";
    const unknown = "Unknown Device";
    const customDevice = "Mozilla/5.0 (CustomOS; Device) CustomBrowser/1.0";

    expect(extractDeviceFromUserAgent(empty)).toBe("other");
    expect(extractDeviceFromUserAgent(unknown)).toBe("other");
    expect(extractDeviceFromUserAgent(customDevice)).toBe("other");
  });

  it("should handle Android edge cases", () => {
    // Android device with explicit mobile flag
    const androidMobile =
      "Mozilla/5.0 (Linux; Android 10; Mobile) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.105 Safari/537.36";
    // Android device without mobile flag (should be detected as tablet)
    const androidNonMobile =
      "Mozilla/5.0 (Linux; Android 10) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.105 Safari/537.36";

    expect(extractDeviceFromUserAgent(androidMobile)).toBe("mobile");
    expect(extractDeviceFromUserAgent(androidNonMobile)).toBe("tablet");
  });
});
