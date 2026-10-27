import type { UpToNineHundredSixty } from "$/int/upToNineHundredSixty";

describe("UpToNineHundredSixty", () => {
  it("UpToNineHundredSixty", () => {
    const _: UpToNineHundredSixty[] = [
      950, 951, 952, 953, 954, 955, 956, 957, 958, 959, 960,
    ];
    expect(_).toEqual(_);
  });
  it("949 extends UpToNineHundredSixty", () => {
    const _: 949 extends UpToNineHundredSixty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("961 extends UpToNineHundredSixty", () => {
    const _: 961 extends UpToNineHundredSixty ? true : false = false;
    expect(_).toEqual(_);
  });
});
