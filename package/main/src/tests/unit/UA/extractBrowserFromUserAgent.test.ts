import { extractBrowserFromUserAgent } from "@/UA/extractBrowserFromUserAgent";

describe("extractBrowserFromUserAgent", () => {
  it("should detect Edge browser", () => {
    const edgeUA =
      "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36 Edg/89.0.774.57";
    const legacyEdgeUA =
      "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36 Edge/89.0.774.57";
    expect(extractBrowserFromUserAgent(edgeUA)).toBe("edge");
    expect(extractBrowserFromUserAgent(legacyEdgeUA)).toBe("edge");
  });

  it("should detect desktop Chrome browser", () => {
    const chromeUA =
      "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36";
    expect(extractBrowserFromUserAgent(chromeUA)).toBe("chrome");
  });

  it("should detect iOS Chrome browser (CriOS)", () => {
    const chromeIosUA =
      "Mozilla/5.0 (iPhone; CPU iPhone OS 14_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/89.0.4389.90 Mobile/15E148 Safari/604.1";
    const chromeIpadUA =
      "Mozilla/5.0 (iPad; CPU OS 14_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/89.0.4389.90 Mobile/15E148 Safari/604.1";
    expect(extractBrowserFromUserAgent(chromeIosUA)).toBe("chrome");
    expect(extractBrowserFromUserAgent(chromeIpadUA)).toBe("chrome");
  });

  it("should detect Firefox browser", () => {
    const firefoxUA =
      "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:88.0) Gecko/20100101 Firefox/88.0";
    const firefoxIosUA =
      "Mozilla/5.0 (iPhone; CPU iPhone OS 14_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) FxiOS/33.0 Mobile/15E148 Safari/605.1.15";
    expect(extractBrowserFromUserAgent(firefoxUA)).toBe("firefox");
    expect(extractBrowserFromUserAgent(firefoxIosUA)).toBe("firefox");
  });

  it("should detect Safari browser", () => {
    const safariUA =
      "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.1 Safari/605.1.15";
    const mobileSafariUA =
      "Mozilla/5.0 (iPhone; CPU iPhone OS 14_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0.3 Mobile/15E148 Safari/604.1";
    expect(extractBrowserFromUserAgent(safariUA)).toBe("safari");
    expect(extractBrowserFromUserAgent(mobileSafariUA)).toBe("safari");
  });

  it("should detect Internet Explorer", () => {
    const ie11UA =
      "Mozilla/5.0 (Windows NT 10.0; WOW64; Trident/7.0; rv:11.0) like Gecko";
    const ie10UA =
      "Mozilla/5.0 (compatible; MSIE 10.0; Windows NT 6.1; Trident/6.0)";
    expect(extractBrowserFromUserAgent(ie11UA)).toBe("ie");
    expect(extractBrowserFromUserAgent(ie10UA)).toBe("ie");
  });

  it("should return 'other' for unknown browsers", () => {
    const operaUA =
      "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36 OPR/75.0.3969.149";
    const unknownUA = "Unknown Browser";
    const emptyUA = "";
    expect(extractBrowserFromUserAgent(operaUA)).toBe("other");
    expect(extractBrowserFromUserAgent(unknownUA)).toBe("other");
    expect(extractBrowserFromUserAgent(emptyUA)).toBe("other");
  });
});
