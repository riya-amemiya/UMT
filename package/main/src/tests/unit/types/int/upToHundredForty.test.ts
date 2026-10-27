import type { UpToHundredForty } from "$/int/upToHundredForty";

describe("UpToHundredForty", () => {
  it("UpToHundredForty", () => {
    const _: UpToHundredForty[] = [
      130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140,
    ];
    expect(_).toEqual(_);
  });
  it("129 extends UpToHundredForty", () => {
    const _: 129 extends UpToHundredForty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("141 extends UpToHundredForty", () => {
    const _: 141 extends UpToHundredForty ? true : false = false;
    expect(_).toEqual(_);
  });
});
