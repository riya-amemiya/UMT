import type { SimplifiedUserAgentInfoBrowser } from "$/ua/simplifiedUserAgentInfoBrowser";

describe("SimplifiedUserAgentInfoBrowser", () => {
  it("SimplifiedUserAgentInfoBrowser", () => {
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
  it('"opera" extends SimplifiedUserAgentInfoBrowser', () => {
    const _: "opera" extends SimplifiedUserAgentInfoBrowser ? true : false =
      false;
    expect(_).toEqual(_);
  });
});
