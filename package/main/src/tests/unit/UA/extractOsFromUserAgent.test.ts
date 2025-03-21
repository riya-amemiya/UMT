import { extractOsFromUserAgent } from "@/UA/extractOsFromUserAgent";

describe("extractOsFromUserAgent", () => {
  it("should detect iOS devices", () => {
    const iphone =
      "Mozilla/5.0 (iPhone; CPU iPhone OS 14_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0.3 Mobile/15E148 Safari/604.1";
    const ipad =
      "Mozilla/5.0 (iPad; CPU OS 14_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0 Mobile/15E148 Safari/604.1";
    const ipod =
      "Mozilla/5.0 (iPod touch; CPU iPhone OS 14_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0 Mobile/15E148 Safari/604.1";

    expect(extractOsFromUserAgent(iphone)).toBe("ios");
    expect(extractOsFromUserAgent(ipad)).toBe("ios");
    expect(extractOsFromUserAgent(ipod)).toBe("ios");
  });

  it("should detect Android devices", () => {
    const androidPhone =
      "Mozilla/5.0 (Linux; Android 10; SM-A505FN) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.105 Mobile Safari/537.36";
    const androidTablet =
      "Mozilla/5.0 (Linux; Android 11; SM-T870) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.105 Safari/537.36";
    const androidTV =
      "Mozilla/5.0 (Linux; Android 10; AndroidTV) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.105 Safari/537.36";

    expect(extractOsFromUserAgent(androidPhone)).toBe("android");
    expect(extractOsFromUserAgent(androidTablet)).toBe("android");
    expect(extractOsFromUserAgent(androidTV)).toBe("android");
  });

  it("should detect macOS devices", () => {
    const macOsSafari =
      "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.1 Safari/605.1.15";
    const macOsChrome =
      "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36";
    const macOsFirefox =
      "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:88.0) Gecko/20100101 Firefox/88.0";

    expect(extractOsFromUserAgent(macOsSafari)).toBe("macos");
    expect(extractOsFromUserAgent(macOsChrome)).toBe("macos");
    expect(extractOsFromUserAgent(macOsFirefox)).toBe("macos");
  });

  it("should detect Windows devices", () => {
    const win10Edge =
      "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36 Edg/89.0.774.57";
    const win10Chrome =
      "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36";
    const win7IE =
      "Mozilla/5.0 (Windows NT 6.1; WOW64; Trident/7.0; rv:11.0) like Gecko";
    const win32UA =
      "Mozilla/5.0 (Win32; x86) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36";

    expect(extractOsFromUserAgent(win10Edge)).toBe("windows");
    expect(extractOsFromUserAgent(win10Chrome)).toBe("windows");
    expect(extractOsFromUserAgent(win7IE)).toBe("windows");
    expect(extractOsFromUserAgent(win32UA)).toBe("windows");
  });

  it("should detect Linux devices", () => {
    const ubuntuChrome =
      "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36";
    const fedoraFirefox =
      "Mozilla/5.0 (X11; Linux x86_64; rv:88.0) Gecko/20100101 Firefox/88.0";
    const genericLinux =
      "Mozilla/5.0 (Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36";

    expect(extractOsFromUserAgent(ubuntuChrome)).toBe("linux");
    expect(extractOsFromUserAgent(fedoraFirefox)).toBe("linux");
    expect(extractOsFromUserAgent(genericLinux)).toBe("linux");
  });

  it("should handle unknown operating systems", () => {
    const empty = "";
    const unknown = "Unknown OS";
    const customOs = "Mozilla/5.0 (CustomOS) CustomBrowser/1.0";
    const bot =
      "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)";

    expect(extractOsFromUserAgent(empty)).toBe("other");
    expect(extractOsFromUserAgent(unknown)).toBe("other");
    expect(extractOsFromUserAgent(customOs)).toBe("other");
    expect(extractOsFromUserAgent(bot)).toBe("other");
  });
});
