import type { NOR } from "$/logic/nor";

describe("NOR", () => {
  it("NOR<1, 1>", () => {
    const _: NOR<1, 1> = false;
    expect(_).toEqual(_);
  });
  it("NOR<1, 0>", () => {
    const _: NOR<1, 0> = false;
    expect(_).toEqual(_);
  });
  it("NOR<0, 1>", () => {
    const _: NOR<0, 1> = false;
    expect(_).toEqual(_);
  });
  it("NOR<0, 0>", () => {
    const _: NOR<0, 0> = true;
    expect(_).toEqual(_);
  });
});
