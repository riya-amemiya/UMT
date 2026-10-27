import type { TimeUnitShort } from "$/time/timeUnitShort";

describe("TimeUnitShort", () => {
  it("TimeUnitShort", () => {
    const _: TimeUnitShort[] = ["ms", "s", "m", "h"];
    expect(_).toEqual(_);
  });
  it('"d" extends TimeUnitShort', () => {
    const _: "d" extends TimeUnitShort ? true : false = false;
    expect(_).toEqual(_);
  });
});
