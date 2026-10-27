import type {
  BinaryToDecimal,
  BinaryToDecimalParser,
} from "$/logic/binaryToDecimal";

describe("BinaryToDecimal", () => {
  it('BinaryToDecimal<"00000101">', () => {
    const _: BinaryToDecimal<"00000101"> = 5;
    expect(_).toEqual(_);
  });
  it('BinaryToDecimal<"00000000">', () => {
    const _: BinaryToDecimal<"00000000"> = 0;
    expect(_).toEqual(_);
  });
  it('BinaryToDecimal<"11111111">', () => {
    const _: BinaryToDecimal<"11111111"> = "-1";
    expect(_).toEqual(_);
  });
  it('BinaryToDecimal<"11111110">', () => {
    const _: BinaryToDecimal<"11111110"> = "-2";
    expect(_).toEqual(_);
  });
  it('BinaryToDecimalParser<"101">', () => {
    const _: BinaryToDecimalParser<"101"> = 5;
    expect(_).toEqual(_);
  });
});
