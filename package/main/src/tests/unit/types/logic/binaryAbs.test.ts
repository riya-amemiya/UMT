import type { BinaryAbs } from "$/logic/binaryAbs";

describe("BinaryAbs", () => {
  it('BinaryAbs<"11111111">', () => {
    const _: BinaryAbs<"11111111"> = "00000001";
    expect(_).toEqual(_);
  });
  it('BinaryAbs<"00000101">', () => {
    const _: BinaryAbs<"00000101"> = "00000101";
    expect(_).toEqual(_);
  });
});
