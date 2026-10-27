import type { XOR } from "$/logic/xor";

describe("XOR", () => {
  it("XOR<1, 1>", () => {
    const _: XOR<1, 1> = false;
    expect(_).toEqual(_);
  });
  it("XOR<1, 0>", () => {
    const _: XOR<1, 0> = true;
    expect(_).toEqual(_);
  });
  it("XOR<0, 1>", () => {
    const _: XOR<0, 1> = true;
    expect(_).toEqual(_);
  });
  it("XOR<0, 0>", () => {
    const _: XOR<0, 0> = false;
    expect(_).toEqual(_);
  });
});
