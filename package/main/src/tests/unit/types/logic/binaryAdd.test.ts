import type { BinaryAdd, BinaryAddParser } from "$/logic/binaryAdd";

describe("BinaryAdd", () => {
  it('BinaryAdd<"00000001", "00000001">', () => {
    const _: BinaryAdd<"00000001", "00000001"> = "00000010";
    expect(_).toEqual(_);
  });
  it('BinaryAdd<"00000011", "00000101">', () => {
    const _: BinaryAdd<"00000011", "00000101"> = "00001000";
    expect(_).toEqual(_);
  });
  it('BinaryAdd<"11111111", "00000001">', () => {
    const _: BinaryAdd<"11111111", "00000001"> = "00000000";
    expect(_).toEqual(_);
  });
  it('BinaryAddParser<"0", "0", "0">', () => {
    const _: BinaryAddParser<"0", "0", "0"> = "00";
    expect(_).toEqual(_);
  });
  it('BinaryAddParser<"0", "0", "1">', () => {
    const _: BinaryAddParser<"0", "0", "1"> = "01";
    expect(_).toEqual(_);
  });
  it('BinaryAddParser<"0", "1", "0">', () => {
    const _: BinaryAddParser<"0", "1", "0"> = "01";
    expect(_).toEqual(_);
  });
  it('BinaryAddParser<"0", "1", "1">', () => {
    const _: BinaryAddParser<"0", "1", "1"> = "10";
    expect(_).toEqual(_);
  });
  it('BinaryAddParser<"1", "0", "0">', () => {
    const _: BinaryAddParser<"1", "0", "0"> = "01";
    expect(_).toEqual(_);
  });
  it('BinaryAddParser<"1", "0", "1">', () => {
    const _: BinaryAddParser<"1", "0", "1"> = "10";
    expect(_).toEqual(_);
  });
  it('BinaryAddParser<"1", "1", "0">', () => {
    const _: BinaryAddParser<"1", "1", "0"> = "10";
    expect(_).toEqual(_);
  });
  it('BinaryAddParser<"1", "1", "1">', () => {
    const _: BinaryAddParser<"1", "1", "1"> = "11";
    expect(_).toEqual(_);
  });
  it('BinaryAddParser<"", "">', () => {
    const _: BinaryAddParser<"", ""> = "0";
    expect(_).toEqual(_);
  });
});
