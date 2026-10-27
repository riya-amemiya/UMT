import type { DeepPartial } from "$/logic/deepPartial";

describe("DeepPartial", () => {
  it("DeepPartial<{ a: { b: number }; c: string }>", () => {
    const _: DeepPartial<{ a: { b: number }; c: string }> = { a: {} };
    expect(_).toEqual(_);
  });
  it("DeepPartial<number>", () => {
    const _: DeepPartial<number> = 1;
    expect(_).toEqual(_);
  });
});
