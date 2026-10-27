import type { BinaryXnor, BinaryXnorParser } from "$/logic/binaryXnor";

describe("BinaryXnor", () => {
  it('BinaryXnor<"11001100", "10101010">', () => {
    const _: BinaryXnor<"11001100", "10101010"> = "10011001";
    expect(_).toEqual(_);
  });
  it('BinaryXnorParser<"1", "">', () => {
    const _: BinaryXnorParser<"1", ""> = "";
    expect(_).toEqual(_);
  });
  it('BinaryXnorParser<"", "1">', () => {
    const _: BinaryXnorParser<"", "1"> = "";
    expect(_).toEqual(_);
  });
});
