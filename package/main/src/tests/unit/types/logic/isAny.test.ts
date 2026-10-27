import type { IsAny } from "$/logic/isAny";

describe("IsAny", () => {
  it("IsAny<ReturnType<typeof JSON.parse>>", () => {
    const _: IsAny<ReturnType<typeof JSON.parse>> = true;
    expect(_).toEqual(_);
  });
  it("IsAny<unknown>", () => {
    const _: IsAny<unknown> = false;
    expect(_).toEqual(_);
  });
});
