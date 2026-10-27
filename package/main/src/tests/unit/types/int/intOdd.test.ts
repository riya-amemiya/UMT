import type { IntOdd } from "$/int/intOdd";

describe("IntOdd", () => {
  it("IntOdd", () => {
    const _: IntOdd[] = [1, 3, 5, 7, 9];
    expect(_).toEqual(_);
  });
  it("0 extends IntOdd", () => {
    const _: 0 extends IntOdd ? true : false = false;
    expect(_).toEqual(_);
  });
});
