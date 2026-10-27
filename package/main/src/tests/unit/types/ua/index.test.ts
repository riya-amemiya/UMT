import type { SimplifiedUserAgentInfoOs } from "$/ua/index";

describe("ua index", () => {
  it("SimplifiedUserAgentInfoOs", () => {
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
  it('"freebsd" extends SimplifiedUserAgentInfoOs', () => {
    const _: "freebsd" extends SimplifiedUserAgentInfoOs ? true : false = false;
    expect(_).toEqual(_);
  });
});
