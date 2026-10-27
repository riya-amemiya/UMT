import type {
  BinaryHalfAdder,
  BinaryHalfAdderParser,
} from "$/logic/binaryHalfAdder";

describe("BinaryHalfAdder", () => {
  it('BinaryHalfAdder<"1", "1">', () => {
    const _: BinaryHalfAdder<"1", "1"> = "10";
    expect(_).toEqual(_);
  });
  it('BinaryHalfAdder<"1", "0">', () => {
    const _: BinaryHalfAdder<"1", "0"> = "01";
    expect(_).toEqual(_);
  });
  it('BinaryHalfAdder<"0", "1">', () => {
    const _: BinaryHalfAdder<"0", "1"> = "01";
    expect(_).toEqual(_);
  });
  it('BinaryHalfAdder<"0", "0">', () => {
    const _: BinaryHalfAdder<"0", "0"> = "00";
    expect(_).toEqual(_);
  });
  it('BinaryHalfAdderParser<"1", "1", "1">', () => {
    const _: BinaryHalfAdderParser<"1", "1", "1"> = "10";
    expect(_).toEqual(_);
  });
});
