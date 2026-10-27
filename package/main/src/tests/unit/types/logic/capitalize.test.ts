import type { Capitalize } from "$/logic/capitalize";

describe("Capitalize", () => {
  it('Capitalize<"hello">', () => {
    const _: Capitalize<"hello"> = "Hello";
    expect(_).toEqual(_);
  });
  it('Capitalize<"">', () => {
    const _: Capitalize<""> = "";
    expect(_).toEqual(_);
  });
});
