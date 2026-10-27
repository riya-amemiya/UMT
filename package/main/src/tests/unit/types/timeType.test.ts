import type { TimeUnit } from "$/time/timeUnit";
import type { TimeUnitShort } from "$/time/timeUnitShort";

describe("timeType", () => {
  it("timeUnit", () => {
    const _: TimeUnit[] = ["milliseconds", "seconds", "minutes", "hours"];
    expect(_).toEqual(_);
  });
  it("timeUnitShort", () => {
    const _: TimeUnitShort[] = ["ms", "s", "m", "h"];
    expect(_).toEqual(_);
  });
});
