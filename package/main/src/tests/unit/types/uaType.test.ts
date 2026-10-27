import type { SimplifiedUserAgentInfo } from "$/ua/simplifiedUserAgentInfo";
import type { SimplifiedUserAgentInfoBrowser } from "$/ua/simplifiedUserAgentInfoBrowser";
import type { SimplifiedUserAgentInfoDevice } from "$/ua/simplifiedUserAgentInfoDevice";
import type { SimplifiedUserAgentInfoOs } from "$/ua/simplifiedUserAgentInfoOs";

describe("uaType", () => {
  it("simplifiedUserAgentInfoBrowser", () => {
    const _: SimplifiedUserAgentInfoBrowser[] = [
      "chrome",
      "firefox",
      "safari",
      "edge",
      "ie",
      "other",
    ];
    expect(_).toEqual(_);
  });
  it("simplifiedUserAgentInfoDevice", () => {
    const _: SimplifiedUserAgentInfoDevice[] = [
      "mobile",
      "tablet",
      "desktop",
      "bot",
      "other",
    ];
    expect(_).toEqual(_);
  });
  it("simplifiedUserAgentInfoOs", () => {
    const _: SimplifiedUserAgentInfoOs[] = [
      "ios",
      "android",
      "macos",
      "windows",
      "linux",
      "other",
    ];
    expect(_).toEqual(_);
  });
  it("simplifiedUserAgentInfo", () => {
    const _: SimplifiedUserAgentInfo = {
      os: "macos",
      browser: "safari",
      device: "desktop",
    };
    expect(_).toEqual(_);
  });
});
