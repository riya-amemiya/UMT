import type { SimplifiedUserAgentInfo } from "$/ua/simplifiedUserAgentInfo";

describe("SimplifiedUserAgentInfo", () => {
  it("SimplifiedUserAgentInfo", () => {
    const _: SimplifiedUserAgentInfo = {
      os: "macos",
      browser: "safari",
      device: "desktop",
    };
    expect(_).toEqual(_);
  });
  it("{ os: string } extends SimplifiedUserAgentInfo", () => {
    const _: { os: string } extends SimplifiedUserAgentInfo ? true : false =
      false;
    expect(_).toEqual(_);
  });
});
