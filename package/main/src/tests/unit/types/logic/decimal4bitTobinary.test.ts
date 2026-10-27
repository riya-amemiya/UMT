import type { Decimal4bitTobinary } from "$/logic/decimal4bitTobinary";

describe("Decimal4bitTobinary", () => {
  it('Decimal4bitTobinary<"0">', () => {
    const _: Decimal4bitTobinary<"0"> = "0000";
    expect(_).toEqual(_);
  });
  it('Decimal4bitTobinary<"1">', () => {
    const _: Decimal4bitTobinary<"1"> = "0001";
    expect(_).toEqual(_);
  });
  it('Decimal4bitTobinary<"2">', () => {
    const _: Decimal4bitTobinary<"2"> = "0010";
    expect(_).toEqual(_);
  });
  it('Decimal4bitTobinary<"3">', () => {
    const _: Decimal4bitTobinary<"3"> = "0011";
    expect(_).toEqual(_);
  });
  it('Decimal4bitTobinary<"4">', () => {
    const _: Decimal4bitTobinary<"4"> = "0100";
    expect(_).toEqual(_);
  });
  it('Decimal4bitTobinary<"5">', () => {
    const _: Decimal4bitTobinary<"5"> = "0101";
    expect(_).toEqual(_);
  });
  it('Decimal4bitTobinary<"6">', () => {
    const _: Decimal4bitTobinary<"6"> = "0110";
    expect(_).toEqual(_);
  });
  it('Decimal4bitTobinary<"7">', () => {
    const _: Decimal4bitTobinary<"7"> = "0111";
    expect(_).toEqual(_);
  });
  it('Decimal4bitTobinary<"8">', () => {
    const _: Decimal4bitTobinary<"8"> = "1000";
    expect(_).toEqual(_);
  });
  it('Decimal4bitTobinary<"9">', () => {
    const _: Decimal4bitTobinary<"9"> = "1001";
    expect(_).toEqual(_);
  });
  it('Decimal4bitTobinary<"10">', () => {
    const _: Decimal4bitTobinary<"10"> = "1010";
    expect(_).toEqual(_);
  });
  it('Decimal4bitTobinary<"11">', () => {
    const _: Decimal4bitTobinary<"11"> = "1011";
    expect(_).toEqual(_);
  });
  it('Decimal4bitTobinary<"12">', () => {
    const _: Decimal4bitTobinary<"12"> = "1100";
    expect(_).toEqual(_);
  });
  it('Decimal4bitTobinary<"13">', () => {
    const _: Decimal4bitTobinary<"13"> = "1101";
    expect(_).toEqual(_);
  });
  it('Decimal4bitTobinary<"14">', () => {
    const _: Decimal4bitTobinary<"14"> = "1110";
    expect(_).toEqual(_);
  });
  it('Decimal4bitTobinary<"15">', () => {
    const _: Decimal4bitTobinary<"15"> = "1111";
    expect(_).toEqual(_);
  });
  it('Decimal4bitTobinary<"16">', () => {
    const _: [Decimal4bitTobinary<"16">] extends [never] ? true : false = true;
    expect(_).toEqual(_);
  });
});
