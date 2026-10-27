import type { UpToNineHundredFifty } from "$/int/upToNineHundredFifty";

describe("UpToNineHundredFifty", () => {
  it("UpToNineHundredFifty", () => {
    const _: UpToNineHundredFifty[] = [
      940, 941, 942, 943, 944, 945, 946, 947, 948, 949, 950,
    ];
    expect(_).toEqual(_);
  });
  it("939 extends UpToNineHundredFifty", () => {
    const _: 939 extends UpToNineHundredFifty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("951 extends UpToNineHundredFifty", () => {
    const _: 951 extends UpToNineHundredFifty ? true : false = false;
    expect(_).toEqual(_);
  });
});
