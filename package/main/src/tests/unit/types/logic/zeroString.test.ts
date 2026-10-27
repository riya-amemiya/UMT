import type { ZeroString } from "$/logic/zeroString";

describe("ZeroString", () => {
  it("ZeroString<3>", () => {
    const _: ZeroString<3> = "000";
    expect(_).toEqual(_);
  });
  it("ZeroString<0>", () => {
    const _: ZeroString<0> = "";
    expect(_).toEqual(_);
  });
});
