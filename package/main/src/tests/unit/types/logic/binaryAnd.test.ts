import type { BinaryAnd, BinaryAndParser } from "$/logic/binaryAnd";

describe("BinaryAnd", () => {
  it('BinaryAnd<"11001100", "10101010">', () => {
    const _: BinaryAnd<"11001100", "10101010"> = "10001000";
    expect(_).toEqual(_);
  });
  it('BinaryAndParser<"1", "">', () => {
    const _: BinaryAndParser<"1", ""> = "";
    expect(_).toEqual(_);
  });
  it('BinaryAndParser<"", "1">', () => {
    const _: BinaryAndParser<"", "1"> = "";
    expect(_).toEqual(_);
  });
});
