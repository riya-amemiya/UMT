import type { SimplifiedUserAgentInfoDevice } from "$/ua/simplifiedUserAgentInfoDevice";

describe("SimplifiedUserAgentInfoDevice", () => {
  it("SimplifiedUserAgentInfoDevice", () => {
    const _: SimplifiedUserAgentInfoDevice[] = [
      "mobile",
      "tablet",
      "desktop",
      "bot",
      "other",
    ];
    expect(_).toEqual(_);
  });
  it('"tv" extends SimplifiedUserAgentInfoDevice', () => {
    const _: "tv" extends SimplifiedUserAgentInfoDevice ? true : false = false;
    expect(_).toEqual(_);
  });
});
