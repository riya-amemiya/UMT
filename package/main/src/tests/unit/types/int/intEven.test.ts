import type { IntEven } from "$/int/intEven";

describe("IntEven", () => {
  it("IntEven", () => {
    const _: IntEven[] = [0, 2, 4, 6, 8];
    expect(_).toEqual(_);
  });
  it("1 extends IntEven", () => {
    const _: 1 extends IntEven ? true : false = false;
    expect(_).toEqual(_);
  });
});
