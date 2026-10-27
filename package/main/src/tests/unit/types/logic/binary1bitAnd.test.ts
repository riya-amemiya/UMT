import type { Binary1bitAnd, Binary1bitAndParser } from "$/logic/binary1bitAnd";

describe("Binary1bitAnd", () => {
  it('Binary1bitAnd<"1", "1">', () => {
    const _: Binary1bitAnd<"1", "1"> = "1";
    expect(_).toEqual(_);
  });
  it('Binary1bitAnd<"1", "0">', () => {
    const _: Binary1bitAnd<"1", "0"> = "0";
    expect(_).toEqual(_);
  });
  it('Binary1bitAnd<"0", "1">', () => {
    const _: Binary1bitAnd<"0", "1"> = "0";
    expect(_).toEqual(_);
  });
  it('Binary1bitAnd<"0", "0">', () => {
    const _: Binary1bitAnd<"0", "0"> = "0";
    expect(_).toEqual(_);
  });
  it('Binary1bitAndParser<"1", "1">', () => {
    const _: Binary1bitAndParser<"1", "1"> = "1";
    expect(_).toEqual(_);
  });
});
