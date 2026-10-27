import type {
  Binary1bitNand,
  Binary1bitNandParser,
} from "$/logic/binary1bitNand";

describe("Binary1bitNand", () => {
  it('Binary1bitNand<"1", "1">', () => {
    const _: Binary1bitNand<"1", "1"> = "0";
    expect(_).toEqual(_);
  });
  it('Binary1bitNand<"1", "0">', () => {
    const _: Binary1bitNand<"1", "0"> = "1";
    expect(_).toEqual(_);
  });
  it('Binary1bitNand<"0", "1">', () => {
    const _: Binary1bitNand<"0", "1"> = "1";
    expect(_).toEqual(_);
  });
  it('Binary1bitNand<"0", "0">', () => {
    const _: Binary1bitNand<"0", "0"> = "1";
    expect(_).toEqual(_);
  });
  it('Binary1bitNandParser<"1", "1">', () => {
    const _: Binary1bitNandParser<"1", "1"> = "0";
    expect(_).toEqual(_);
  });
});
