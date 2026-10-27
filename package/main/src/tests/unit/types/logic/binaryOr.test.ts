import type { BinaryOr, BinaryOrParser } from "$/logic/binaryOr";

describe("BinaryOr", () => {
  it('BinaryOr<"11001100", "10101010">', () => {
    const _: BinaryOr<"11001100", "10101010"> = "11101110";
    expect(_).toEqual(_);
  });
  it('BinaryOrParser<"1", "">', () => {
    const _: BinaryOrParser<"1", ""> = "";
    expect(_).toEqual(_);
  });
  it('BinaryOrParser<"", "1">', () => {
    const _: BinaryOrParser<"", "1"> = "";
    expect(_).toEqual(_);
  });
});
