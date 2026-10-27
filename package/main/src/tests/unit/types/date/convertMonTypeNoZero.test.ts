import type { ConvertMonTypeNoZero } from "$/date/convertMonTypeNoZero";

describe("ConvertMonTypeNoZero", () => {
  it('ConvertMonTypeNoZero<"01">', () => {
    const _: ConvertMonTypeNoZero<"01"> = "1";
    expect(_).toEqual(_);
  });
  it('ConvertMonTypeNoZero<"10">', () => {
    const _: ConvertMonTypeNoZero<"10"> = "10";
    expect(_).toEqual(_);
  });
});
