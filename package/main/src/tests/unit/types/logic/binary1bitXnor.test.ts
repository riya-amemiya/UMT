import type {
  Binary1bitXnor,
  Binary1bitXnorParser,
} from "$/logic/binary1bitXnor";

describe("Binary1bitXnor", () => {
  it('Binary1bitXnor<"1", "1">', () => {
    const _: Binary1bitXnor<"1", "1"> = "1";
    expect(_).toEqual(_);
  });
  it('Binary1bitXnor<"1", "0">', () => {
    const _: Binary1bitXnor<"1", "0"> = "0";
    expect(_).toEqual(_);
  });
  it('Binary1bitXnor<"0", "1">', () => {
    const _: Binary1bitXnor<"0", "1"> = "0";
    expect(_).toEqual(_);
  });
  it('Binary1bitXnor<"0", "0">', () => {
    const _: Binary1bitXnor<"0", "0"> = "1";
    expect(_).toEqual(_);
  });
  it('Binary1bitXnorParser<"0", "0">', () => {
    const _: Binary1bitXnorParser<"0", "0"> = "1";
    expect(_).toEqual(_);
  });
});
