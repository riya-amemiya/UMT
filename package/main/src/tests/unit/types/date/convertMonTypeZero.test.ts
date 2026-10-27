import type { ConvertMonTypeZero } from "$/date/convertMonTypeZero";

describe("ConvertMonTypeZero", () => {
  it("ConvertMonTypeZero<1>", () => {
    const _: ConvertMonTypeZero<1> = "01";
    expect(_).toEqual(_);
  });
  it('ConvertMonTypeZero<"9">', () => {
    const _: ConvertMonTypeZero<"9"> = "09";
    expect(_).toEqual(_);
  });
  it("ConvertMonTypeZero<10>", () => {
    const _: ConvertMonTypeZero<10> = 10;
    expect(_).toEqual(_);
  });
  it('ConvertMonTypeZero<"11">', () => {
    const _: ConvertMonTypeZero<"11"> = "11";
    expect(_).toEqual(_);
  });
  it('ConvertMonTypeZero<"12">', () => {
    const _: ConvertMonTypeZero<"12"> = "12";
    expect(_).toEqual(_);
  });
});
