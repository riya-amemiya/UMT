import type { FourDigit } from "$/int/fourDigit";

describe("FourDigit", () => {
  it("FourDigit with every leading digit", () => {
    const _: FourDigit[] = [
      "1000",
      "2000",
      "3000",
      "4000",
      "5000",
      "6000",
      "7000",
      "8000",
      "9000",
    ];
    expect(_).toEqual(_);
  });
  it("FourDigit with every trailing digit", () => {
    const _: FourDigit[] = [
      "1000",
      "1111",
      "1222",
      "1333",
      "1444",
      "1555",
      "1666",
      "1777",
      "1888",
      "1999",
    ];
    expect(_).toEqual(_);
  });
  it('"0999" extends FourDigit', () => {
    const _: "0999" extends FourDigit ? true : false = false;
    expect(_).toEqual(_);
  });
  it('"999" extends FourDigit', () => {
    const _: "999" extends FourDigit ? true : false = false;
    expect(_).toEqual(_);
  });
  it('"10000" extends FourDigit', () => {
    const _: "10000" extends FourDigit ? true : false = false;
    expect(_).toEqual(_);
  });
});
