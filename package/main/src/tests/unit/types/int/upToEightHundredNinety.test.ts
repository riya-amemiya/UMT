import type { UpToEightHundredNinety } from "$/int/upToEightHundredNinety";

describe("UpToEightHundredNinety", () => {
  it("UpToEightHundredNinety", () => {
    const _: UpToEightHundredNinety[] = [
      880, 881, 882, 883, 884, 885, 886, 887, 888, 889, 890,
    ];
    expect(_).toEqual(_);
  });
  it("879 extends UpToEightHundredNinety", () => {
    const _: 879 extends UpToEightHundredNinety ? true : false = false;
    expect(_).toEqual(_);
  });
  it("891 extends UpToEightHundredNinety", () => {
    const _: 891 extends UpToEightHundredNinety ? true : false = false;
    expect(_).toEqual(_);
  });
});
