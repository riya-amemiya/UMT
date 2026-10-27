import type { Binary1bitNor, Binary1bitNorParser } from "$/logic/binary1bitNor";

describe("Binary1bitNor", () => {
  it('Binary1bitNor<"1", "1">', () => {
    const _: Binary1bitNor<"1", "1"> = "0";
    expect(_).toEqual(_);
  });
  it('Binary1bitNor<"1", "0">', () => {
    const _: Binary1bitNor<"1", "0"> = "0";
    expect(_).toEqual(_);
  });
  it('Binary1bitNor<"0", "1">', () => {
    const _: Binary1bitNor<"0", "1"> = "0";
    expect(_).toEqual(_);
  });
  it('Binary1bitNor<"0", "0">', () => {
    const _: Binary1bitNor<"0", "0"> = "1";
    expect(_).toEqual(_);
  });
  it('Binary1bitNorParser<"0", "0">', () => {
    const _: Binary1bitNorParser<"0", "0"> = "1";
    expect(_).toEqual(_);
  });
});
