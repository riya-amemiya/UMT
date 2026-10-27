import type { TimeUnit } from "$/time/timeUnit";

describe("TimeUnit", () => {
  it("TimeUnit", () => {
    const _: TimeUnit[] = ["milliseconds", "seconds", "minutes", "hours"];
    expect(_).toEqual(_);
  });
  it('"days" extends TimeUnit', () => {
    const _: "days" extends TimeUnit ? true : false = false;
    expect(_).toEqual(_);
  });
});
