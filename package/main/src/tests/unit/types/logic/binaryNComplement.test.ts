import type { BinaryNComplement } from "$/logic/binaryNComplement";

describe("BinaryNComplement", () => {
  it('BinaryNComplement<"0001">', () => {
    const _: BinaryNComplement<"0001"> = "1111";
    expect(_).toEqual(_);
  });
  it('BinaryNComplement<"0110">', () => {
    const _: BinaryNComplement<"0110"> = "1010";
    expect(_).toEqual(_);
  });
});
