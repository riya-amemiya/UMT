import type {
  BinaryFullAdder,
  BinaryFullAdderParser,
} from "$/logic/binaryFullAdder";

describe("BinaryFullAdder", () => {
  it('BinaryFullAdder<"0011", "0001">', () => {
    const _: BinaryFullAdder<"0011", "0001"> = "0100";
    expect(_).toEqual(_);
  });
  it('BinaryFullAdder<"01", "001">', () => {
    const _: [BinaryFullAdder<"01", "001">] extends [never] ? true : false =
      true;
    expect(_).toEqual(_);
  });
  it('BinaryFullAdderParser<"1", "1">', () => {
    const _: BinaryFullAdderParser<"1", "1"> = "01";
    expect(_).toEqual(_);
  });
  it('BinaryFullAdderParser<"1", "">', () => {
    const _: [BinaryFullAdderParser<"1", "">] extends [never] ? true : false =
      true;
    expect(_).toEqual(_);
  });
  it('BinaryFullAdderParser<"", "">', () => {
    const _: BinaryFullAdderParser<"", ""> = "0";
    expect(_).toEqual(_);
  });
});
