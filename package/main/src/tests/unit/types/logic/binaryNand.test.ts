import type { BinaryNand, BinaryNandParser } from "$/logic/binaryNand";

describe("BinaryNand", () => {
  it('BinaryNand<"11001100", "10101010">', () => {
    const _: BinaryNand<"11001100", "10101010"> = "01110111";
    expect(_).toEqual(_);
  });
  it('BinaryNandParser<"1", "">', () => {
    const _: BinaryNandParser<"1", ""> = "";
    expect(_).toEqual(_);
  });
  it('BinaryNandParser<"", "1">', () => {
    const _: BinaryNandParser<"", "1"> = "";
    expect(_).toEqual(_);
  });
});
