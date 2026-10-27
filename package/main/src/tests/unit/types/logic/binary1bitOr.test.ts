import type { Binary1bitor, Binary1bitOrParser } from "$/logic/binary1bitOr";

describe("Binary1bitor", () => {
  it('Binary1bitor<"1", "1">', () => {
    const _: Binary1bitor<"1", "1"> = "1";
    expect(_).toEqual(_);
  });
  it('Binary1bitor<"1", "0">', () => {
    const _: Binary1bitor<"1", "0"> = "1";
    expect(_).toEqual(_);
  });
  it('Binary1bitor<"0", "1">', () => {
    const _: Binary1bitor<"0", "1"> = "1";
    expect(_).toEqual(_);
  });
  it('Binary1bitor<"0", "0">', () => {
    const _: Binary1bitor<"0", "0"> = "0";
    expect(_).toEqual(_);
  });
  it('Binary1bitOrParser<"0", "0">', () => {
    const _: Binary1bitOrParser<"0", "0"> = "0";
    expect(_).toEqual(_);
  });
});
