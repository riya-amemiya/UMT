import type { Hex4bitToDecimal } from "$/logic/hex4bitToDecimal";

describe("Hex4bitToDecimal", () => {
  it('Hex4bitToDecimal<"9">', () => {
    const _: Hex4bitToDecimal<"9"> = "9";
    expect(_).toEqual(_);
  });
  it('Hex4bitToDecimal<"A">', () => {
    const _: Hex4bitToDecimal<"A"> = "10";
    expect(_).toEqual(_);
  });
  it('Hex4bitToDecimal<"B">', () => {
    const _: Hex4bitToDecimal<"B"> = "11";
    expect(_).toEqual(_);
  });
  it('Hex4bitToDecimal<"C">', () => {
    const _: Hex4bitToDecimal<"C"> = "12";
    expect(_).toEqual(_);
  });
  it('Hex4bitToDecimal<"D">', () => {
    const _: Hex4bitToDecimal<"D"> = "13";
    expect(_).toEqual(_);
  });
  it('Hex4bitToDecimal<"E">', () => {
    const _: Hex4bitToDecimal<"E"> = "14";
    expect(_).toEqual(_);
  });
  it('Hex4bitToDecimal<"F">', () => {
    const _: Hex4bitToDecimal<"F"> = "15";
    expect(_).toEqual(_);
  });
});
