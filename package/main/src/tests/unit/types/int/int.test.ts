import type { Int } from "$/int/int";

describe("Int", () => {
  it("Int", () => {
    const _: Int[] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    expect(_).toEqual(_);
  });
  it("10 extends Int", () => {
    const _: 10 extends Int ? true : false = false;
    expect(_).toEqual(_);
  });
});
