import { parseUserAgent } from "@/UA/parseUserAgent";

describe("parseUserAgent", () => {
  it("should detect iOS mobile with Safari", () => {
    const ua =
      "Mozilla/5.0 (iPhone; CPU iPhone OS 14_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0.3 Mobile/15E148 Safari/604.1";
    const result = parseUserAgent(ua);
    expect(result).toEqual({ os: "ios", browser: "safari", device: "mobile" });
  });

  it("should detect Android mobile with Chrome", () => {
    const ua =
      "Mozilla/5.0 (Linux; Android 10; SM-A505FN) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.105 Mobile Safari/537.36";
    const result = parseUserAgent(ua);
    expect(result).toEqual({
      os: "android",
      browser: "chrome",
      device: "mobile",
    });
  });

  it("should detect macOS desktop with Firefox", () => {
    const ua =
      "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:88.0) Gecko/20100101 Firefox/88.0";
    const result = parseUserAgent(ua);
    expect(result).toEqual({
      os: "macos",
      browser: "firefox",
      device: "desktop",
    });
  });

  it("should detect Windows desktop with Edge", () => {
    const ua =
      "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36 Edg/89.0.774.57";
    const result = parseUserAgent(ua);
    expect(result).toEqual({
      os: "windows",
      browser: "edge",
      device: "desktop",
    });
  });

  it("should detect Linux desktop with Chrome", () => {
    const ua =
      "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36";
    const result = parseUserAgent(ua);
    expect(result).toEqual({
      os: "linux",
      browser: "chrome",
      device: "desktop",
    });
  });

  it("should detect iPad as tablet", () => {
    const ua =
      "Mozilla/5.0 (iPad; CPU OS 14_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0 Mobile/15E148 Safari/604.1";
    const result = parseUserAgent(ua);
    expect(result).toEqual({ os: "ios", browser: "safari", device: "tablet" });
  });

  it("should detect Android tablet", () => {
    const ua =
      "Mozilla/5.0 (Linux; Android 11; SM-T870) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.105 Safari/537.36";
    const result = parseUserAgent(ua);
    expect(result).toEqual({
      os: "android",
      browser: "chrome",
      device: "tablet",
    });
  });

  it("should detect Internet Explorer on Windows", () => {
    const ua =
      "Mozilla/5.0 (Windows NT 10.0; WOW64; Trident/7.0; rv:11.0) like Gecko";
    const result = parseUserAgent(ua);
    expect(result).toEqual({ os: "windows", browser: "ie", device: "desktop" });
  });

  it("should detect Googlebot", () => {
    const ua =
      "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)";
    const result = parseUserAgent(ua);
    expect(result).toEqual({ os: "other", browser: "other", device: "bot" });
  });

  it("should handle unknown user agent", () => {
    const ua = "Unknown User Agent String";
    const result = parseUserAgent(ua);
    expect(result).toEqual({ os: "other", browser: "other", device: "other" });
  });
});
