import type { BinaryToHex, BinaryToHexParser } from "$/logic/binaryToHex";

describe("BinaryToHex", () => {
  it('BinaryToHex<"11111111">', () => {
    const _: BinaryToHex<"11111111"> = "FF";
    expect(_).toEqual(_);
  });
  it('BinaryToHex<"00001010">', () => {
    const _: BinaryToHex<"00001010"> = "0A";
    expect(_).toEqual(_);
  });
  it('BinaryToHexParser<"1">', () => {
    const _: BinaryToHexParser<"1"> = "";
    expect(_).toEqual(_);
  });
  it('BinaryToHexParser<"">', () => {
    const _: BinaryToHexParser<""> = "";
    expect(_).toEqual(_);
  });
});
