import type {
  BinaryComplement,
  BinaryComplementParser,
} from "$/logic/binaryComplement";

describe("BinaryComplement", () => {
  it('BinaryComplement<"00000001">', () => {
    const _: BinaryComplement<"00000001"> = "11111111";
    expect(_).toEqual(_);
  });
  it('BinaryComplement<"11111111">', () => {
    const _: BinaryComplement<"11111111"> = "00000001";
    expect(_).toEqual(_);
  });
  it('BinaryComplementParser<"1010">', () => {
    const _: BinaryComplementParser<"1010"> = "0101";
    expect(_).toEqual(_);
  });
  it('BinaryComplementParser<"">', () => {
    const _: BinaryComplementParser<""> = "";
    expect(_).toEqual(_);
  });
});
