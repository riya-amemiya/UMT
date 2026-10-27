import type { BinaryXor, BinaryXorParser } from "$/logic/binaryXor";

describe("BinaryXor", () => {
  it('BinaryXor<"11001100", "10101010">', () => {
    const _: BinaryXor<"11001100", "10101010"> = "01100110";
    expect(_).toEqual(_);
  });
  it('BinaryXorParser<"1", "">', () => {
    const _: BinaryXorParser<"1", ""> = "";
    expect(_).toEqual(_);
  });
  it('BinaryXorParser<"", "1">', () => {
    const _: BinaryXorParser<"", "1"> = "";
    expect(_).toEqual(_);
  });
});
