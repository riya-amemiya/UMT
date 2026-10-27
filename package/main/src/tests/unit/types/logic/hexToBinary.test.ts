import type { HexToBinary, HexToBinaryParser } from "$/logic/hexToBinary";

describe("HexToBinary", () => {
  it('HexToBinary<"FF">', () => {
    const _: HexToBinary<"FF"> = "11111111";
    expect(_).toEqual(_);
  });
  it('HexToBinary<"0A">', () => {
    const _: HexToBinary<"0A"> = "00001010";
    expect(_).toEqual(_);
  });
  it('HexToBinaryParser<"A">', () => {
    const _: HexToBinaryParser<"A"> = "1010";
    expect(_).toEqual(_);
  });
  it('HexToBinaryParser<"">', () => {
    const _: HexToBinaryParser<""> = "";
    expect(_).toEqual(_);
  });
});
