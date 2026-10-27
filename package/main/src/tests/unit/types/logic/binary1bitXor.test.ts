import type { Binary1bitXOr, Binary1bitXorParser } from "$/logic/binary1bitXor";

describe("Binary1bitXOr", () => {
  it('Binary1bitXOr<"1", "1">', () => {
    const _: Binary1bitXOr<"1", "1"> = "0";
    expect(_).toEqual(_);
  });
  it('Binary1bitXOr<"1", "0">', () => {
    const _: Binary1bitXOr<"1", "0"> = "1";
    expect(_).toEqual(_);
  });
  it('Binary1bitXOr<"0", "1">', () => {
    const _: Binary1bitXOr<"0", "1"> = "1";
    expect(_).toEqual(_);
  });
  it('Binary1bitXOr<"0", "0">', () => {
    const _: Binary1bitXOr<"0", "0"> = "0";
    expect(_).toEqual(_);
  });
  it('Binary1bitXorParser<"0", "0">', () => {
    const _: Binary1bitXorParser<"0", "0"> = "0";
    expect(_).toEqual(_);
  });
});
