import type { UpToEighty } from "$/int/upToEighty";

describe("UpToEighty", () => {
  it("UpToEighty", () => {
    const _: UpToEighty[] = [70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80];
    expect(_).toEqual(_);
  });
  it("69 extends UpToEighty", () => {
    const _: 69 extends UpToEighty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("81 extends UpToEighty", () => {
    const _: 81 extends UpToEighty ? true : false = false;
    expect(_).toEqual(_);
  });
});
