import type { IntWithoutZero } from "$/int/intWithoutZero";

describe("IntWithoutZero", () => {
  it("IntWithoutZero", () => {
    const _: IntWithoutZero[] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    expect(_).toEqual(_);
  });
  it("0 extends IntWithoutZero", () => {
    const _: 0 extends IntWithoutZero ? true : false = false;
    expect(_).toEqual(_);
  });
});
