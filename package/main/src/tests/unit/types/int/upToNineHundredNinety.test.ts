import type { UpToNineHundredNinety } from "$/int/upToNineHundredNinety";

describe("UpToNineHundredNinety", () => {
  it("UpToNineHundredNinety", () => {
    const _: UpToNineHundredNinety[] = [
      980, 981, 982, 983, 984, 985, 986, 987, 988, 989, 990,
    ];
    expect(_).toEqual(_);
  });
  it("979 extends UpToNineHundredNinety", () => {
    const _: 979 extends UpToNineHundredNinety ? true : false = false;
    expect(_).toEqual(_);
  });
  it("991 extends UpToNineHundredNinety", () => {
    const _: 991 extends UpToNineHundredNinety ? true : false = false;
    expect(_).toEqual(_);
  });
});
