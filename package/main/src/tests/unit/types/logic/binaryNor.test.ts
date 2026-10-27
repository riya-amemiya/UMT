import type { BinaryNor, BinaryNorParser } from "$/logic/binaryNor";

describe("BinaryNor", () => {
  it('BinaryNor<"11001100", "10101010">', () => {
    const _: BinaryNor<"11001100", "10101010"> = "00010001";
    expect(_).toEqual(_);
  });
  it('BinaryNorParser<"1", "">', () => {
    const _: BinaryNorParser<"1", ""> = "";
    expect(_).toEqual(_);
  });
  it('BinaryNorParser<"", "1">', () => {
    const _: BinaryNorParser<"", "1"> = "";
    expect(_).toEqual(_);
  });
});
