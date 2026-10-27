import type { BinaryNot, BinaryNotParser } from "$/logic/binaryNot";

describe("BinaryNot", () => {
  it('BinaryNot<"1">', () => {
    const _: BinaryNot<"1"> = "0";
    expect(_).toEqual(_);
  });
  it('BinaryNot<"0">', () => {
    const _: BinaryNot<"0"> = "1";
    expect(_).toEqual(_);
  });
  it('BinaryNotParser<"1">', () => {
    const _: BinaryNotParser<"1"> = "0";
    expect(_).toEqual(_);
  });
  it('BinaryNotParser<"0">', () => {
    const _: BinaryNotParser<"0"> = "1";
    expect(_).toEqual(_);
  });
});
