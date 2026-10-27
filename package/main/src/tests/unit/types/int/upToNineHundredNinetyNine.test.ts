import type { UpToNineHundredNinetyNine } from "$/int/upToNineHundredNinetyNine";

describe("UpToNineHundredNinetyNine", () => {
  it("UpToNineHundredNinetyNine", () => {
    const _: UpToNineHundredNinetyNine[] = [
      990, 991, 992, 993, 994, 995, 996, 997, 998, 999,
    ];
    expect(_).toEqual(_);
  });
  it("989 extends UpToNineHundredNinetyNine", () => {
    const _: 989 extends UpToNineHundredNinetyNine ? true : false = false;
    expect(_).toEqual(_);
  });
  it("1000 extends UpToNineHundredNinetyNine", () => {
    const _: 1000 extends UpToNineHundredNinetyNine ? true : false = false;
    expect(_).toEqual(_);
  });
});
