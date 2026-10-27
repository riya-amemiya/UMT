import type { Decimal4bitToHex } from "$/logic/decimal4bitToHex";

describe("Decimal4bitToHex", () => {
  it('Decimal4bitToHex<"9">', () => {
    const _: Decimal4bitToHex<"9"> = "9";
    expect(_).toEqual(_);
  });
  it('Decimal4bitToHex<"10">', () => {
    const _: Decimal4bitToHex<"10"> = "A";
    expect(_).toEqual(_);
  });
  it('Decimal4bitToHex<"11">', () => {
    const _: Decimal4bitToHex<"11"> = "B";
    expect(_).toEqual(_);
  });
  it('Decimal4bitToHex<"12">', () => {
    const _: Decimal4bitToHex<"12"> = "C";
    expect(_).toEqual(_);
  });
  it('Decimal4bitToHex<"13">', () => {
    const _: Decimal4bitToHex<"13"> = "D";
    expect(_).toEqual(_);
  });
  it('Decimal4bitToHex<"14">', () => {
    const _: Decimal4bitToHex<"14"> = "E";
    expect(_).toEqual(_);
  });
  it('Decimal4bitToHex<"15">', () => {
    const _: Decimal4bitToHex<"15"> = "F";
    expect(_).toEqual(_);
  });
});
